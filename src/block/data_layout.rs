use enum_gen::layout;

//
// This file contains a bunch of "tables" that assign names to known data ranges.
// It is generated via a procedural macro.
// The enums actually get replaced with structs with nested functions due to the fact
// that an enum can't have multiple values, but with the way Rust handles byte slices,
// there needs to be multiple values since one entry's end is another entry's start.
//
// Each enum is to be filled with entries similar to the following:
// #[ahead(NUMBER_OF_BYTES_FOR_THIS_VALUE)] ValueName
// and two functions are generated as a result:
// ValueNameStart() which returns the beginning of the range, and ValueNameEnd() which returns
// the end of the range according to the number you provided.
//

#[layout(BlockLayoutGeneric)]
enum BlockLayoutGeneric {
    #[ahead(4)] BlockSize,
    #[ahead(4)] BlockType,
    #[ahead(4)] BlockID,
    #[ahead(4)] Filler0,
}

#[layout(StackDataLayout)]
enum StackDataLayout {
    #[ahead(4)] BlockSize,
    #[ahead(4)] BlockType,
    #[ahead(4)] BlockID,
    #[ahead(4)] Filler0,
    #[ahead(4)] HyperCardFormat,
    #[ahead(4)] DataFork,
    #[ahead(4)] BlockSize2,
    #[ahead(4)] Unk1,
    #[ahead(4)] MaximumEver,
    #[ahead(4)] BackgroundNum,
    #[ahead(4)] FirstBackgroundID,
    #[ahead(4)] CardNum,
    #[ahead(4)] FirstCardID,
    #[ahead(4)] ListID,
    #[ahead(4)] FreeBlockNum,
    #[ahead(4)] FreeBlockSize,
    #[ahead(4)] PrintBlockID,
    #[ahead(4)] PasswordHash,
    #[ahead(2)] UserLevel,
    #[ahead(2)] ProtAlignmentShortOne,
    #[ahead(2)] ProtFlags,
    #[ahead(2)] ProtAlignmentShortEnd,
    #[ahead(16)] SkipAhead16,
    #[ahead(4)] HyperCardVersionAtCreation,
    #[ahead(4)] HyperCardVersionAtLastCompacting,
    #[ahead(4)] HyperCardVersionAtLastModificationSinceLastCompacting,
    #[ahead(4)] HyperCardVersionAtLastModification,
    #[ahead(4)] Checksum,
    #[ahead(4)] MarkedCardNum,
    #[ahead(2)] CardWindowTop,
    #[ahead(2)] CardWindowLeft,
    #[ahead(2)] CardWindowBottom,
    #[ahead(2)] CardWindowRight,
    #[ahead(2)] ScreenTop,
    #[ahead(2)] ScreenLeft,
    #[ahead(2)] ScreenBottom,
    #[ahead(2)] ScreenRight,
    #[ahead(2)] XCoord,
    #[ahead(2)] YCoord,
    #[ahead(2)] Unk2,
    #[ahead(2)] Unk3,
    #[ahead(288)] SkipAhead288,
    #[ahead(4)] FontTableID,
    #[ahead(4)] StyleTableID,
    #[ahead(2)] Height,
    #[ahead(2)] Width,
    #[ahead(2)] Unk4,
    #[ahead(2)] Unk5,
    #[ahead(256)] SkipAhead256,
    #[ahead(320)] PatternTable,
}

#[layout(BitmapLayout)]
enum BitmapLayout {
    #[ahead(4)] BlockSize,
    #[ahead(4)] BlockType,
    #[ahead(4)] BlockID,
    #[ahead(4)] Filler0,
    #[ahead(8)] UnknownGroup,
    #[ahead(2)] CardTop,
    #[ahead(2)] CardLeft,
    #[ahead(2)] CardBottom,
    #[ahead(2)] CardRight,
    #[ahead(2)] MaskTop,
    #[ahead(2)] MaskLeft,
    #[ahead(2)] MaskBottom,
    #[ahead(2)] MaskRight,
    #[ahead(2)] ImageTop,
    #[ahead(2)] ImageLeft,
    #[ahead(2)] ImageBottom,
    #[ahead(2)] ImageRight,
    #[ahead(8)] UnknownGroup2,
    #[ahead(4)] MaskDataSize,
    #[ahead(4)] ImageDataSize
}