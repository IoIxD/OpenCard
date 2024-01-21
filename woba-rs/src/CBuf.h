/*
 *  CBuf.h
 *  stackimport
 *
 *  Created by Mr. Z. on 03/31/10.
 *  Copyright 2010 Mr Z. All rights reserved.
 *
 */

#pragma once

#include <stdlib.h>
#include <stdint.h>
#include <string>

struct shared_buffer
{
	char *mBuffer;
	size_t mSize;
	int mRefCount;
};

class CBuf
{
public:
	explicit CBuf(size_t inSize = 0);
	CBuf(const CBuf &inTemplate, size_t startOffs = 0, size_t amount = SIZE_MAX);
	~CBuf();

	size_t size();
	void resize(size_t inSize);

	void memcpy(size_t toOffs, const char *fromPtr, size_t fromOffs, size_t amount);
	void memcpy(size_t toOffs, const CBuf &fromPtr, size_t fromOffs = 0, size_t amount = SIZE_MAX);

	const char operator[](int idx) const;
	char &operator[](int idx);

	char *buf(size_t offs = 0, size_t amount = SIZE_MAX);
	const char *buf(size_t offs = 0, size_t amount = SIZE_MAX) const;

	void xornstr(size_t dstOffs, char *src, size_t srcOffs, size_t amount);
	void xornstr(size_t dstOffs, const CBuf &src, size_t srcOffs, size_t amount);

	void shiftnstr(size_t dstOffs, int amount, int shiftAmount);

	size_t size() const { return mShared->mSize; };

	int16_t int16at(size_t offs) const
	{
		int16_t *theBuf = (int16_t *)buf(offs, sizeof(int16_t));
		return *theBuf;
	};
	int32_t int32at(size_t offs) const
	{
		int32_t *theBuf = (int32_t *)buf(offs, sizeof(int32_t));
		return *theBuf;
	};

	uint16_t uint16at(size_t offs) const
	{
		uint16_t *theBuf = (uint16_t *)buf(offs, sizeof(uint16_t));
		return *theBuf;
	};
	uint32_t uint32at(size_t offs) const
	{
		uint32_t *theBuf = (uint32_t *)buf(offs, sizeof(uint32_t));
		return *theBuf;
	};

	bool hasdata(size_t offs, size_t amount) { return (mShared->mBuffer != NULL) && (amount + offs) <= mShared->mSize; };

	void tofile(const std::string &fpath);

	void debug_print()
	{
		if (!mShared)
			printf("NULL\n");
		else
		{
			printf("CBuf %p { size = %zd, refCount = %d, \"%-*s\" }\n", this, mShared->mSize, mShared->mRefCount, (int)mShared->mSize, mShared->mBuffer);
		}
	};

	virtual CBuf &operator=(const CBuf &inTemplate);

protected:
	void alloc_buffer(size_t amount);
	void release_buffer();
	void make_buffer_exclusive();

protected:
	shared_buffer *mShared;
};

extern "C"
{
	CBuf cbuf_new_with_size(size_t inSize = 0) { return CBuf(inSize); };
	CBuf cbuf_new_with_uhh(const CBuf &inTemplate, size_t startOffs = 0, size_t amount = SIZE_MAX)
	{
		return CBuf();
	};
	void cbuf_drop(CBuf *cbuf) { delete (cbuf); }

	void cbuf_resize(CBuf *cbuf, size_t inSize) { cbuf->resize(inSize); };

	void cbuf_memcpy(CBuf *cbuf, size_t toOffs, const CBuf &fromPtr, size_t fromOffs = 0, size_t amount = SIZE_MAX)
	{
		cbuf->memcpy(toOffs, fromPtr, fromOffs, amount);
	}

	char &cbuf_index(CBuf *cbuf, int idx)
	{
		return cbuf->operator[](idx);
	};

	char *cbuf_buf_none(CBuf *cbuf)
	{
		return cbuf->buf();
	};
	char *cbuf_buf_w_offs(CBuf *cbuf, size_t offs)
	{
		return cbuf->buf(offs);
	};
	char *cbuf_buf_w_amount(CBuf *cbuf, size_t amount)
	{
		return cbuf->buf(0, amount);
	};
	char *cbuf_buf_w_both(CBuf *cbuf, size_t offs, size_t amount)
	{
		return cbuf->buf(offs, amount);
	};

	void cbuf_xornstr_char(CBuf *cbuf, size_t dstOffs, char *src, size_t srcOffs, size_t amount)
	{
		return cbuf->xornstr(dstOffs, src, srcOffs, amount);
	};
	void cbuf_xornstr_buf(CBuf *cbuf, size_t dstOffs, const CBuf &src, size_t srcOffs, size_t amount)
	{
		return cbuf->xornstr(dstOffs, src, srcOffs, amount);
	};

	void cbuf_shiftnstr(CBuf *cbuf, size_t dstOffs, int amount, int shiftAmount)
	{
		return cbuf->shiftnstr(dstOffs, amount, shiftAmount);
	};

	size_t cbuf_size(CBuf *cbuf) { return cbuf->size(); };

	int16_t cbuf_int16at(CBuf *cbuf, size_t offs)
	{
		cbuf->int16at(offs);
	};
	int32_t cbuf_int32at(CBuf *cbuf, size_t offs)
	{
		cbuf->int32at(offs);
	};

	uint16_t cbuf_uint16at(CBuf *cbuf, size_t offs)
	{
		cbuf->uint16at(offs);
	};
	uint32_t cbuf_uint32at(CBuf *cbuf, size_t offs)
	{
		cbuf->uint32at(offs);
	};

	bool cbuf_hasdata(CBuf *cbuf, size_t offs, size_t amount) { return cbuf->hasdata(offs, amount); };

	void cbuf_tofile(CBuf *cbuf, const std::string &fpath)
	{
		return cbuf->tofile(fpath);
	};

	void cbuf_debug_print(CBuf *cbuf)
	{
		cbuf->debug_print();
	};

	CBuf cbuf_set_to(CBuf *cbuf, CBuf &inTemplate)
	{
		return cbuf->operator=(inTemplate);
	}
}