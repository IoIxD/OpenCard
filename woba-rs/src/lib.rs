use eyre::eyre;
use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use image::{DynamicImage, GrayImage};
use std::error::Error;
use std::ffi;
use std::io::Cursor;

mod cbuf;
mod pic;

use cbuf::CBuf;
use pic::{get_error_str, new_picture_with_params, picture, woba_decode};

pub fn decode(b: &[u8]) -> Result<picture, eyre::ErrReport> {
    let p: picture;
    unsafe {
        p = new_picture_with_params(0, 0, 0, 0);
        woba_decode(&p, &ffi::CStr::from_bytes_with_nul_unchecked(b));

        let err = get_error_str();
        if err.to_bytes().len() > 1 && err.to_bytes().len() < i32::MAX as usize {
            return Err(eyre!(err.to_str()?));
        }
    };
    Ok(p)
}

pub fn htons(u: u16) -> u16 {
    u.to_be()
}
pub fn ntohs(u: u16) -> u16 {
    u16::from_be(u)
}

macro_rules! int16_at {
    ($woba:ident, $pos:literal) => {
        ntohs($woba[$pos] as u16)
    };
}

macro_rules! int32_at {
    ($woba:ident, $pos:literal) => {
        ntohs($woba[$pos] as u16)
    };
}

pub fn min(a: usize, b: usize) -> usize {
    if a < b {
        a
    } else {
        b
    }
}

pub unsafe fn woba_decode_rust(p: &mut picture, woba: &mut [i8]) {
    ///*#if DEBUGOUTPUT
    //std::cout << "===== NEXT BMAP =====" << endl;
    //#endif*/
    let mut totalRectTop = 0;
    let mut totalRectLeft = 0;
    let mut totalRectBottom = 0;
    let mut totalRectRight = 0; /* total rectangle for whole picture */
    let mut maskBoundRectTop = 0;
    let mut maskBoundRectLeft = 0;
    let mut maskBoundRectBottom = 0;
    let mut maskBoundRectRight = 0; /* mask bounding rect */
    let mut pictureBoundRectTop = 0;
    let mut pictureBoundRectLeft = 0;
    let mut pictureBoundRectBottom = 0;
    let mut pictureBoundRectRight = 0; /* picture bounding rect */
    let mut maskDataLength = 0;
    let mut pictureDataLength = 0; /* mask and picture data length */

    let mut bx = 0;
    let mut bx8 = 0;
    let mut x = 0;
    let mut y = 0;
    let mut rowwidth = 0;
    let mut rowwidth8 = 0;
    let mut height = 0;
    let mut dx = 0;
    let mut dy = 0;
    let mut repeat = 1;
    let mut patternbuffer = CBuf::new_with_size(8);
    let mut buffer1 = CBuf::new();
    let mut buffer2 = CBuf::new();
    let mut i = 0;
    let mut j = 0;

    let mut opcode = 0;
    let mut operand = 0;
    let mut operandata = CBuf::new_with_size(256);
    let mut numberOfZeroBytes = 0;
    let mut numberOfDataBytes = 0_usize;
    let mut k = 0;

    /*
        -16 0 - block size & type (already stripped)
        -8  8 - block ID & filler (already stripped)
        8  16 - something
        12 24 - total rect
        20 32 - mask rect
        28 40 - picture rect
        36 48 - nothing
        48 56 - length
        52 64 - start of mask (or bitmap if mask length == 0)
    */
    const MASK_START: usize = 52;

    //#define int16_at!(woba,pos)	ntohs(*(u_int16_t*)(woba+pos))
    //#define int32_at!(woba,pos)	ntohl(*(u_int32_t*)(woba+pos))

    totalRectTop = int16_at!(woba, 12);
    totalRectLeft = int16_at!(woba, 14);
    totalRectBottom = int16_at!(woba, 16);
    totalRectRight = int16_at!(woba, 18);
    maskBoundRectTop = int16_at!(woba, 20);
    maskBoundRectLeft = int16_at!(woba, 22);
    maskBoundRectBottom = int16_at!(woba, 24);
    maskBoundRectRight = int16_at!(woba, 26);
    pictureBoundRectTop = int16_at!(woba, 28);
    pictureBoundRectLeft = int16_at!(woba, 30);
    pictureBoundRectBottom = int16_at!(woba, 32);
    pictureBoundRectRight = int16_at!(woba, 34);
    maskDataLength = int32_at!(woba, 44);
    pictureDataLength = int32_at!(woba, 48);

    /*#if DEBUGOUTPUT
    std::cout << "Total Rect: " << totalRectLeft << "," << totalRectTop << "," << totalRectRight << "," << totalRectBottom << endl;
    std::cout << "Bitmap Rect: " << pictureBoundRectLeft << "," << pictureBoundRectTop << "," << pictureBoundRectRight << "," << pictureBoundRectBottom << endl;
    std::cout << "Mask Rect: " << maskBoundRectLeft << "," << maskBoundRectTop << "," << maskBoundRectRight << "," << maskBoundRectBottom << endl;
    std::cout << "Bitmap Size: " << pictureDataLength << endl;
    std::cout << "Mask Size: " << maskDataLength << endl;
    #endif*/

    p.__directcopybmptomask(); /* clear the mask to zero */
    i = MASK_START;

    /* decode mask */
    if (maskDataLength != 0) {
        bx8 = maskBoundRectLeft & (!0x1F); // Get high 11 bits (mask out low 5 bits).
        bx = bx8 / 8; // Bitshift by 3?
        x = 0;
        y = maskBoundRectTop;
        rowwidth8 = {
            if (maskBoundRectRight & 0x1F == 1) {
                (maskBoundRectRight | 0x1F) + 1
            } else {
                maskBoundRectRight
            }
        } - (maskBoundRectLeft & (!0x1F));
        rowwidth = rowwidth8 / 8;
        height = maskBoundRectBottom - maskBoundRectTop;
        dx = 0;
        dy = 0;
        repeat = 1;

        // Build a 50% grey checkerboard pattern:
        patternbuffer[0] = 170;
        patternbuffer[2] = 170;
        patternbuffer[4] = 170;
        patternbuffer[6] = 170; // Even rows: 170 == 10101010 binary.
        patternbuffer[1] = 85;
        patternbuffer[3] = 85;
        patternbuffer[5] = 85;
        patternbuffer[7] = 85; // Odd rows:   85 == 01010101 binary.

        // Make both buffers large enough to hold a full row of pixels:
        &buffer1.resize(rowwidth as usize);
        &buffer2.resize(rowwidth as usize);

        j = 0;

        /*#if DEBUGOUTPUT
        std::cout << "DECODE MASK:" << endl;
        std::cout << "BX8: " << bx8 << endl << "BX: " << bx << endl << "X: " << x << endl << "Y: " << y << endl;
        std::cout << "RW8: " << rowwidth8 << endl << "RW: " << rowwidth << endl << "H: " << height << endl;
        #endif*/

        while (j < maskDataLength) {
            opcode = woba[i] as usize;
            /*#if DEBUGOUTPUT
            std::cout << "Opcode: " << __hex(opcode) << endl;
            std::cout << "Repeat: " << repeat << endl;
            std::cout << "i: " << i << endl << "j: " << j << endl;
            std::cout << "x: " << x << endl << "y: " << y << endl;
            std::cout << "dx: " << dx << endl << "dy: " << dy << endl;
            #endif*/

            i += 1;
            j += 1;
            if ((opcode & 0x80) == 0) {
                /* zeros followed by data */
                numberOfDataBytes = opcode >> 4; // nd = number of data bytes?
                numberOfZeroBytes = opcode & 15; // nz = number of zeroes?

                /*#if DEBUGOUTPUT
                std::cout << "nd: " << numberOfDataBytes << endl << "nz: " << numberOfZeroBytes << endl;
                #endif*/

                if numberOfDataBytes != 0 {
                    &operandata.memcpy(0, woba, i, numberOfDataBytes as usize);
                    i += numberOfDataBytes as usize;
                    j += numberOfDataBytes as u16;
                }

                while repeat != 0 {
                    let mut k = numberOfZeroBytes;
                    while k > 0 {
                        buffer1[x as i32] = 0;
                        x += 1;
                        k -= 1;
                    }
                    &buffer1.memcpy(x, &operandata, 0, numberOfDataBytes as usize);
                    x += numberOfDataBytes;
                    repeat -= 1;
                }
                repeat = 1;
            } else if ((opcode & 0xE0) == 0xC0) {
                /* opcode & 1F * 8 bytes of data */
                numberOfDataBytes = (opcode & 0x1F) * 8;
                /*#if DEBUGOUTPUT
                std::cout << "nd: " << numberOfDataBytes << endl;
                #endif*/
                if (numberOfDataBytes != 0) {
                    &operandata.memcpy(0, woba, i, numberOfDataBytes as usize);
                    i += numberOfDataBytes as usize;
                    j += numberOfDataBytes as u16;
                }
                while repeat != 0 {
                    &buffer1.memcpy(x, &operandata, 0, numberOfDataBytes);
                    x += numberOfDataBytes;
                    repeat -= 1;
                }
                repeat = 1;
            } else if ((opcode & 0xE0) == 0xE0) {
                /* opcode & 1F * 16 bytes of zero */
                numberOfZeroBytes = (opcode & 0x1F) * 16;
                /*#if DEBUGOUTPUT
                std::cout << "nz: " << numberOfZeroBytes << endl;
                #endif*/
                while repeat != 0 {
                    let mut k = numberOfZeroBytes;
                    while k > 0 {
                        if (x as usize) < buffer1.size() {
                            buffer1[x as i32] = 0;
                        }
                        x += 1;
                    }
                    repeat -= 1;
                }
                repeat = 1;
            }

            if ((opcode & 0xE0) == 0xA0)
            // Repeat the next opcode a certain number of times.
            {
                /* repeat opcode */
                repeat = (opcode & 0x1F);
            } else {
                match (opcode) {
                    0x80 => {
                        /* uncompressed data */
                        x = 0;
                        while repeat != 0 {
                            p.maskmemcopyin(&mut woba[i..], bx8, y, Some(rowwidth));
                            y += 1;
                            repeat -= 1;
                        }
                        repeat = 1;
                        i += rowwidth as usize;
                        j += rowwidth;
                    }
                    0x81 => {
                        /* white row */
                        x = 0;
                        while repeat != 0 {
                            p.maskmemfill(0, bx8, y, Some(rowwidth));
                            y += 1;
                            repeat -= 1;
                        }
                        repeat = 1;
                    }
                    0x82 => {
                        /* black row */
                        x = 0;
                        while repeat != 0 {
                            p.maskmemfill(0xFF, bx8, y, Some(rowwidth));
                            y += 1;
                            repeat -= 1;
                        }
                        repeat = 1;
                    }
                    0x83 => {
                        /* pattern */
                        operand = woba[i].clone();
                        /*#if DEBUGOUTPUT
                        std::cout << "patt: " << __hex(operand) << endl;
                        #endif*/
                        i += 1;
                        j += 1;
                        x = 0;
                        while repeat != 0 {
                            patternbuffer[y & 7] = operand;
                            p.maskmemfill(operand, bx8, y, Some(rowwidth));
                            y += 1;
                            repeat -= 1;
                        }
                        repeat = 1;
                    }
                    0x84 => {
                        /* last pattern */
                        x = 0;
                        while repeat != 0 {
                            operand = patternbuffer[y & 7];
                            /*#if DEBUGOUTPUT
                            std::cout << "patt: " << __hex(operand) << endl;
                            #endif*/
                            p.maskmemfill(operand, bx8, y, Some(rowwidth));
                            y += 1;
                            repeat -= 1;
                        }
                        repeat = 1;
                    }
                    0x85 => {
                        /* previous row */
                        x = 0;
                        while repeat != 0 {
                            p.maskcopyrow(y, y - 1);
                            y += 1;
                            repeat -= 1;
                        }
                        repeat = 1;
                    }
                    0x86 => {
                        /* two rows back */
                        x = 0;
                        while repeat != 0 {
                            p.maskcopyrow(y, y - 2);
                            y += 1;
                            repeat -= 1;
                        }
                        repeat = 1;
                    }
                    0x87 => {
                        /* three rows back */
                        x = 0;
                        while repeat != 0 {
                            p.maskcopyrow(y, y - 3);
                            y += 1;
                            repeat -= 1;
                        }
                        repeat = 1;
                    }
                    0x88 => {
                        dx = 16;
                        dy = 0;
                    }

                    0x89 => {
                        dx = 0;
                        dy = 0;
                    }

                    0x8A => {
                        dx = 0;
                        dy = 1;
                    }

                    0x8B => {
                        dx = 0;
                        dy = 2;
                    }

                    0x8C => {
                        dx = 1;
                        dy = 0;
                    }

                    0x8D => {
                        dx = 1;
                        dy = 1;
                    }

                    0x8E => {
                        dx = 2;
                        dy = 2;
                    }

                    0x8F => {
                        dx = 8;
                        dy = 0;
                    }

                    _ => {
                        /* it's not a repeat or a whole row */
                        if (x >= rowwidth as usize) {
                            x = 0;
                            if (dx != 0) {
                                &buffer2.memcpy(0, &buffer1, 0, rowwidth);
                                let mut k = rowwidth8 / dx;
                                while k > 0 {
                                    &buffer2.shiftnstr(0, rowwidth, dx);
                                    &buffer1.xornstr(0, &buffer2, 0, rowwidth);
                                    k -= 1;
                                }
                            }
                            if (dy != 0) {
                                p.maskmemcopyout_x_y_buf(&mut buffer2, bx8, y - dy, rowwidth);
                                &buffer1.xornstr(0, &buffer2, 0, rowwidth);
                            }
                            let mut a = buffer1.buf(None, None);
                            p.maskmemcopyin(a, bx8, y, Some(rowwidth));
                            y += 1;
                        }
                    }
                }
            }
        }

        &buffer1.resize(0);
        &buffer2.resize(0);
    } else if (maskBoundRectTop | maskBoundRectLeft | maskBoundRectBottom | maskBoundRectRight) != 0
    {
        /* mask is a simple rectangle */
        bx = maskBoundRectLeft / 8;
        x = 0;
        rowwidth = (maskBoundRectRight - maskBoundRectLeft) / 8;
        if (rowwidth > 0) {
            &buffer1.resize(rowwidth.into());
            let mut k = bx;
            while (x < rowwidth.into()) {
                buffer1[x as i32] = 0xFF; // was k as index.
                k += 1;
                x += 1;
            }
            let mut k = maskBoundRectTop;
            while (k < maskBoundRectBottom) {
                p.maskmemcopyin(buffer1.buf(None, None), 0, k, Some(rowwidth));
                k += 1;
            }
            &buffer1.resize(0);
        }
    }

    /* decode bitmap */
    if pictureDataLength != 0 {
        bx8 = pictureBoundRectLeft & (!0x1F);
        bx = bx8 / 8;
        x = 0;
        y = pictureBoundRectTop;
        rowwidth8 = {
            if (pictureBoundRectRight & 0x1F) != 0 {
                (pictureBoundRectRight | 0x1F) + 1
            } else {
                pictureBoundRectRight
            }
        } - (pictureBoundRectLeft & (!0x1F));
        rowwidth = rowwidth8 / 8;
        height = pictureBoundRectBottom - pictureBoundRectTop;
        dx = 0;
        dy = 0;
        repeat = 1;

        // Build a 50% grey pattern, even rows are 10101010 and odd rows are 01010101:
        patternbuffer[0] = 170;
        patternbuffer[2] = 170;
        patternbuffer[4] = 170;
        patternbuffer[6] = 170;
        patternbuffer[1] = 85;
        patternbuffer[3] = 85;
        patternbuffer[5] = 85;
        patternbuffer[7] = 85;

        &buffer1.resize(rowwidth.into());
        &buffer2.resize(rowwidth.into());
        j = 0;

        /*#if DEBUGOUTPUT
        std::cout << "DECODE BITMAP:" << endl;
        std::cout << "BX8: " << bx8 << endl << "BX: " << bx << endl << "X: " << x << endl << "Y: " << y << endl;
        std::cout << "RW8: " << rowwidth8 << endl << "RW: " << rowwidth << endl << "H: " << height << endl;
        #endif*/

        while (j < pictureDataLength) {
            opcode = woba[i] as usize;
            /*#if DEBUGOUTPUT
            std::cout << "Opcode: " << __hex(opcode) << endl;
            std::cout << "Repeat: " << repeat << endl;
            std::cout << "i: " << i << endl << "j: " << j << endl;
            std::cout << "x: " << x << endl << "y: " << y << endl;
            std::cout << "dx: " << dx << endl << "dy: " << dy << endl;
            #endif*/
            i += 1;
            j += 1;
            if ((opcode & 0x80) == 0) {
                /* zeros followed by data */
                numberOfDataBytes = opcode >> 4;
                numberOfZeroBytes = opcode & 15;

                /*#if DEBUGOUTPUT
                std::cout << "nd: " << numberOfDataBytes << endl << "nz: " << numberOfZeroBytes << endl;
                #endif*/

                if numberOfDataBytes != 0 {
                    &operandata.memcpy(0, woba, i, numberOfDataBytes);
                    i += numberOfDataBytes;
                    j += numberOfDataBytes as u16;
                }
                while repeat != 0 {
                    let mut k = numberOfZeroBytes;
                    while k > 0 {
                        buffer1[x as i32] = 0;
                        x += 1;
                        k -= 1;
                    }
                    &buffer1.memcpy(x, &operandata, 0, numberOfDataBytes);
                    x += numberOfDataBytes;
                    repeat -= 1;
                }
                repeat = 1;
            } else if ((opcode & 0xE0) == 0xC0) {
                /* opcode & 1F * 8 bytes of data */
                numberOfDataBytes = (opcode & 0x1F) * 8;
                /*#if DEBUGOUTPUT
                std::cout << "nd: " << numberOfDataBytes << endl;
                #endif*/
                if numberOfDataBytes != 0 {
                    &operandata.memcpy(0, woba, i, numberOfDataBytes as usize);
                    i += numberOfDataBytes as usize;
                    j += numberOfDataBytes as u16;
                }
                while repeat != 0 {
                    let mut a = &buffer1.size() - x;
                    &buffer1.memcpy(x, &operandata, 0, min(a, numberOfDataBytes));
                    x += numberOfDataBytes;
                    repeat -= 1;
                }
                repeat = 1;
            } else if ((opcode & 0xE0) == 0xE0) {
                /* opcode & 1F * 16 bytes of zero */
                numberOfZeroBytes = (opcode & 0x1F) * 16;
                /*#if DEBUGOUTPUT
                std::cout << "nz: " << numberOfZeroBytes << endl;
                #endif*/
                while repeat != 0 {
                    let mut k = numberOfZeroBytes;
                    while (k > 0) {
                        let mut b = &buffer1.size() - 1;
                        buffer1[min(x, b) as i32] = 0;
                        x += 1;
                        k -= 1;
                    }
                    repeat -= 1;
                }
                repeat = 1;
            }

            if ((opcode & 0xE0) == 0xA0) {
                /* repeat opcode */
                repeat = (opcode & 0x1F);
            } else {
                match (opcode) {
                    0x80 => {
                        /* uncompressed data */
                        x = 0;
                        while repeat != 0 {
                            p.memcopyin(&mut woba[i..], bx8, y, Some(rowwidth));
                            y += 1;
                            repeat -= 1;
                        }
                        repeat = 1;
                        i += rowwidth as usize;
                        j += rowwidth;
                    }
                    0x81 => {
                        /* white row */
                        x = 0;
                        while repeat != 0 {
                            p.memfill(0, bx8, y, Some(rowwidth));
                            y += 1;
                            repeat -= 1;
                        }
                        repeat = 1;
                    }
                    0x82 => {
                        /* black row */
                        x = 0;
                        while repeat != 0 {
                            p.memfill(0xFF, bx8, y, Some(rowwidth));
                            y += 1;
                            repeat -= 1;
                        }
                        repeat = 1;
                    }
                    0x83 => {
                        /* pattern */
                        operand = woba[i];
                        /*#if DEBUGOUTPUT
                        std::cout << "patt: " << __hex(operand) << endl;
                        #endif*/
                        i += 1;
                        j += 1;
                        x = 0;
                        while repeat != 0 {
                            patternbuffer[y & 7] = operand;
                            p.memfill(operand, bx8, y, Some(rowwidth));
                            y += 1;
                            repeat -= 1;
                        }
                        repeat = 1;
                    }
                    0x84 => {
                        /* last pattern */
                        x = 0;
                        while repeat != 0 {
                            operand = patternbuffer[y & 7];
                            /*#if DEBUGOUTPUT
                            std::cout << "patt: " << __hex(operand) << endl;
                            #endif*/
                            p.memfill(operand, bx8, y, Some(rowwidth));
                            y += 1;
                            repeat -= 1;
                        }
                        repeat = 1;
                    }
                    0x85 => {
                        /* previous row */
                        x = 0;
                        while repeat != 0 {
                            p.copyrow(y, y - 1);
                            y += 1;
                            repeat -= 1;
                        }
                        repeat = 1;
                    }
                    0x86 => {
                        /* two rows back */
                        x = 0;
                        while repeat != 0 {
                            p.copyrow(y, y - 2);
                            y += 1;
                            repeat -= 1;
                        }
                        repeat = 1;
                    }
                    0x87 => {
                        /* three rows back */
                        x = 0;
                        while repeat != 0 {
                            p.copyrow(y, y - 3);
                            y += 1;
                            repeat -= 1;
                        }
                        repeat = 1;
                    }
                    0x88 => {
                        dx = 16;
                        dy = 0;
                    }

                    0x89 => {
                        dx = 0;
                        dy = 0;
                    }

                    0x8A => {
                        dx = 0;
                        dy = 1;
                    }

                    0x8B => {
                        dx = 0;
                        dy = 2;
                    }

                    0x8C => {
                        dx = 1;
                        dy = 0;
                    }

                    0x8D => {
                        dx = 1;
                        dy = 1;
                    }

                    0x8E => {
                        dx = 2;
                        dy = 2;
                    }

                    0x8F => {
                        dx = 8;
                        dy = 0;
                    }

                    _ => {
                        /* it's not a repeat or a whole row */
                        if (x >= rowwidth.into()) {
                            x = 0;
                            if dx != 0 {
                                &buffer2.memcpy(0, &buffer1, 0, rowwidth);
                                let mut k = rowwidth8 / dx;
                                while k > 0 {
                                    &buffer2.shiftnstr(0, rowwidth, dx);
                                    &buffer1.xornstr(0, &buffer2, 0, rowwidth);
                                    k -= 1;
                                }
                            }
                            if dy != 0 {
                                p.memcopyout_x_y_buf(&mut &buffer2, bx8, y - dy, rowwidth);
                                &buffer1.xornstr(0, &buffer2, 0, rowwidth);
                            }
                            p.memcopyin(buffer1.buf(None, None), bx8, y, Some(rowwidth));
                            y += 1;
                        }
                    }
                }
            }
        }

        &buffer1.resize(0);
        &buffer2.resize(0);
    }

    if (!(maskDataLength
        | maskBoundRectTop
        | maskBoundRectLeft
        | maskBoundRectBottom
        | maskBoundRectRight))
        != 0
    {
        /* mask needs to be copied from picture */
        p.__directcopybmptomask();
    }
}
