/*
 *  CStackFile.cpp
 *  stackimport
 *
 *  Created by Mr. Z. on 10/06/06.
 *  Copyright 2006 Mr Z. All rights reserved.
 *
 */


#include "CStackFile.h"
#include <iostream>
#include <fstream>
#include <sys/stat.h>
#include <unistd.h>
#include "picture.h"
#include "woba.h"
#include "CBuf.h"
#include <arpa/inet.h>
//#include "snd2wav/snd2wav/snd2wav.h"



// Table of C-strings for converting the non-ASCII MacRoman characters (above 127)
//	into the requisite UTF8 byte sequences:
unsigned char	sMacRomanToUTF8Table[128][5] =
{
	{ 0xc3, 0x84, 0x00, 0x00 }, { 0xc3, 0x85, 0x00, 0x00 }, { 0xc3, 0x87, 0x00, 0x00 }, { 0xc3, 0x89, 0x00, 0x00 },
	{ 0xc3, 0x91, 0x00, 0x00 }, { 0xc3, 0x96, 0x00, 0x00 }, { 0xc3, 0x9c, 0x00, 0x00 }, { 0xc3, 0xa1, 0x00, 0x00 },
	{ 0xc3, 0xa0, 0x00, 0x00 }, { 0xc3, 0xa2, 0x00, 0x00 }, { 0xc3, 0xa4, 0x00, 0x00 }, { 0xc3, 0xa3, 0x00, 0x00 },
	{ 0xc3, 0xa5, 0x00, 0x00 }, { 0xc3, 0xa7, 0x00, 0x00 }, { 0xc3, 0xa9, 0x00, 0x00 }, { 0xc3, 0xa8, 0x00, 0x00 },
	{ 0xc3, 0xaa, 0x00, 0x00 }, { 0xc3, 0xab, 0x00, 0x00 }, { 0xc3, 0xad, 0x00, 0x00 }, { 0xc3, 0xac, 0x00, 0x00 },
	{ 0xc3, 0xae, 0x00, 0x00 }, { 0xc3, 0xaf, 0x00, 0x00 }, { 0xc3, 0xb1, 0x00, 0x00 }, { 0xc3, 0xb3, 0x00, 0x00 },
	{ 0xc3, 0xb2, 0x00, 0x00 }, { 0xc3, 0xb4, 0x00, 0x00 }, { 0xc3, 0xb6, 0x00, 0x00 }, { 0xc3, 0xb5, 0x00, 0x00 },
	{ 0xc3, 0xba, 0x00, 0x00 }, { 0xc3, 0xb9, 0x00, 0x00 }, { 0xc3, 0xbb, 0x00, 0x00 }, { 0xc3, 0xbc, 0x00, 0x00 },
	{ 0xe2, 0x80, 0xa0, 0x00 }, { 0xc2, 0xb0, 0x00, 0x00 }, { 0xc2, 0xa2, 0x00, 0x00 }, { 0xc2, 0xa3, 0x00, 0x00 },
	{ 0xc2, 0xa7, 0x00, 0x00 }, { 0xe2, 0x80, 0xa2, 0x00 }, { 0xc2, 0xb6, 0x00, 0x00 }, { 0xc3, 0x9f, 0x00, 0x00 },
	{ 0xc2, 0xae, 0x00, 0x00 }, { 0xc2, 0xa9, 0x00, 0x00 }, { 0xe2, 0x84, 0xa2, 0x00 }, { 0xc2, 0xb4, 0x00, 0x00 },
	{ 0xc2, 0xa8, 0x00, 0x00 }, { 0xe2, 0x89, 0xa0, 0x00 }, { 0xc3, 0x86, 0x00, 0x00 }, { 0xc3, 0x98, 0x00, 0x00 },
	{ 0xe2, 0x88, 0x9e, 0x00 }, { 0xc2, 0xb1, 0x00, 0x00 }, { 0xe2, 0x89, 0xa4, 0x00 }, { 0xe2, 0x89, 0xa5, 0x00 },
	{ 0xc2, 0xa5, 0x00, 0x00 }, { 0xc2, 0xb5, 0x00, 0x00 }, { 0xe2, 0x88, 0x82, 0x00 }, { 0xe2, 0x88, 0x91, 0x00 },
	{ 0xe2, 0x88, 0x8f, 0x00 }, { 0xcf, 0x80, 0x00, 0x00 }, { 0xe2, 0x88, 0xab, 0x00 }, { 0xc2, 0xaa, 0x00, 0x00 },
	{ 0xc2, 0xba, 0x00, 0x00 }, { 0xce, 0xa9, 0x00, 0x00 }, { 0xc3, 0xa6, 0x00, 0x00 }, { 0xc3, 0xb8, 0x00, 0x00 },
	{ 0xc2, 0xbf, 0x00, 0x00 }, { 0xc2, 0xa1, 0x00, 0x00 }, { 0xc2, 0xac, 0x00, 0x00 }, { 0xe2, 0x88, 0x9a, 0x00 },
	{ 0xc6, 0x92, 0x00, 0x00 }, { 0xe2, 0x89, 0x88, 0x00 }, { 0xe2, 0x88, 0x86, 0x00 }, { 0xc2, 0xab, 0x00, 0x00 },
	{ 0xc2, 0xbb, 0x00, 0x00 }, { 0xe2, 0x80, 0xa6, 0x00 }, { 0xc2, 0xa0, 0x00, 0x00 }, { 0xc3, 0x80, 0x00, 0x00 },
	{ 0xc3, 0x83, 0x00, 0x00 }, { 0xc3, 0x95, 0x00, 0x00 }, { 0xc5, 0x92, 0x00, 0x00 }, { 0xc5, 0x93, 0x00, 0x00 },
	{ 0xe2, 0x80, 0x93, 0x00 }, { 0xe2, 0x80, 0x94, 0x00 }, { 0xe2, 0x80, 0x9c, 0x00 }, { 0xe2, 0x80, 0x9d, 0x00 },
	{ 0xe2, 0x80, 0x98, 0x00 }, { 0xe2, 0x80, 0x99, 0x00 }, { 0xc3, 0xb7, 0x00, 0x00 }, { 0xe2, 0x97, 0x8a, 0x00 },
	{ 0xc3, 0xbf, 0x00, 0x00 }, { 0xc5, 0xb8, 0x00, 0x00 }, { 0xe2, 0x81, 0x84, 0x00 }, { 0xe2, 0x82, 0xac, 0x00 },
	{ 0xe2, 0x80, 0xb9, 0x00 }, { 0xe2, 0x80, 0xba, 0x00 }, { 0xef, 0xac, 0x81, 0x00 }, { 0xef, 0xac, 0x82, 0x00 },
	{ 0xe2, 0x80, 0xa1, 0x00 }, { 0xc2, 0xb7, 0x00, 0x00 }, { 0xe2, 0x80, 0x9a, 0x00 }, { 0xe2, 0x80, 0x9e, 0x00 },
	{ 0xe2, 0x80, 0xb0, 0x00 }, { 0xc3, 0x82, 0x00, 0x00 }, { 0xc3, 0x8a, 0x00, 0x00 }, { 0xc3, 0x81, 0x00, 0x00 },
	{ 0xc3, 0x8b, 0x00, 0x00 }, { 0xc3, 0x88, 0x00, 0x00 }, { 0xc3, 0x8d, 0x00, 0x00 }, { 0xc3, 0x8e, 0x00, 0x00 },
	{ 0xc3, 0x8f, 0x00, 0x00 }, { 0xc3, 0x8c, 0x00, 0x00 }, { 0xc3, 0x93, 0x00, 0x00 }, { 0xc3, 0x94, 0x00, 0x00 },
	{ 0xef, 0xa3, 0xbf, 0x00 }, { 0xc3, 0x92, 0x00, 0x00 }, { 0xc3, 0x9a, 0x00, 0x00 }, { 0xc3, 0x9b, 0x00, 0x00 },
	{ 0xc3, 0x99, 0x00, 0x00 }, { 0xc4, 0xb1, 0x00, 0x00 }, { 0xcb, 0x86, 0x00, 0x00 }, { 0xcb, 0x9c, 0x00, 0x00 },
	{ 0xc2, 0xaf, 0x00, 0x00 }, { 0xcb, 0x98, 0x00, 0x00 }, { 0xcb, 0x99, 0x00, 0x00 }, { 0xcb, 0x9a, 0x00, 0x00 },
	{ 0xc2, 0xb8, 0x00, 0x00 }, { 0xcb, 0x9d, 0x00, 0x00 }, { 0xcb, 0x9b, 0x00, 0x00 }, { 0xcb, 0x87, 0x00, 0x00 }
};


const unsigned char*	UniCharFromMacRoman( unsigned char c )
{
	if( c >= 128 )
		return sMacRomanToUTF8Table[ c -128 ];
	else if( c == 0x11 )
	{
		static unsigned char	commandKey[4] = { 0xe2, 0x8c, 0x98, 0 };	// Unicode 0x2318
		return commandKey;
	}
	else
	{
		static unsigned char	asciiStr[2] = { 0, 0 };
		asciiStr[0] = c;
		return asciiStr;
	}
}


void	NumVersionToStr( unsigned char numVersion[4], char outStr[16] )
{
	char	theCh = 'v';
	
	switch( numVersion[2] )
	{
		case 0x20:
			theCh = 'd';
			break;
		case 0x40:
			theCh = 'a';
			break;
		case 0x60:
			theCh = 'b';
			break;
		case 0x80:
			theCh = 'v';
			break;
	}
	
	// NumVersion is Binary-coded decimal, i.e. 0x10 is displayed as 10, not 16 decimal:
	if( numVersion[3] == 0 && (numVersion[1] & 0x0F) == 0 )	// N.N version
		snprintf( outStr, 16, "%x.%x", numVersion[0], (numVersion[1] >> 4) );
	else if( (numVersion[1] & 0x0F) == 0 )	// N.NxN version
		snprintf( outStr, 16, "%x.%x%c%d", numVersion[0], (numVersion[1] >> 4), theCh, numVersion[3] );
	else if( numVersion[3] == 0 )	// N.N.N version
		snprintf( outStr, 16, "%x.%x.%x", numVersion[0], (numVersion[1] >> 4), (numVersion[1] & 0x0F) );
	else	// N.N.NxN version
		snprintf( outStr, 16, "%x.%x.%x%c%d", numVersion[0], (numVersion[1] >> 4), (numVersion[1] & 0x0F), theCh, numVersion[3] );
}



CStackFile::CStackFile()
	: mDumpRawBlockData(false), mStatusMessages(true), mXmlFile(NULL),
	mCardBlockSize(-1), mListBlockID(-1), mMaxProgress(0), mCurrentProgress(0),
	mFontTableBlockID(-1), mStyleTableBlockID(-1), mProgressMessages(true), mDecodeGraphics(true)
{
	
}


bool	CStackFile::LoadStackBlock( int32_t stackID, CBuf& blockData )
{
	if( mStatusMessages )
		fprintf( stdout, "Status: Processing 'STAK' #-1 (%lu bytes)\n", blockData.size() );

	fprintf( mXmlFile, "\t<stack id=\"%d\" file=\"stack_%d.xml\" name=\"", stackID, stackID );
	for( int x = 0; mFileName[x] != 0; x++ )
	{
		char currCh = mFileName[x];
		if( currCh == '"' )
			fprintf( mXmlFile, "%%22" );
		else if( currCh == '\n' )
			fprintf( mXmlFile, "%%0A;" );
		else if( currCh == '\r' )
			fprintf( mXmlFile, "%%0D" );
		else
			fputc( currCh, mXmlFile );	// mFileName comes from POSIX, should already be UTF8, is *definitely* not MacRoman.
	}
	fprintf( mXmlFile, "\" />\n" );
	
	if( mDumpRawBlockData )
	{
		char sfn[256] = { 0 };
		snprintf( sfn, sizeof(sfn), "STAK_%d.data", stackID );
		blockData.tofile( sfn );
	}
	
	fprintf( mStackXmlFile, "\t<id>%d</id>\n", stackID );
	int32_t	numberOfCards = ntohl(blockData.int32at( 32 ));
	fprintf( mStackXmlFile, "\t<cardCount>%d</cardCount>\n", numberOfCards );
	int32_t	cardID = ntohl(blockData.int32at( 36 ));
	fprintf( mStackXmlFile, "\t<cardID>%d</cardID>\n", cardID );
	mListBlockID = ntohl(blockData.int32at( 40 ));
	fprintf( mStackXmlFile, "\t<listID>%d</listID>\n", mListBlockID );
	int16_t	userLevel = ntohs(blockData.int16at( 60 ));
	fprintf( mXmlFile, "\t<userLevel>%d</userLevel>\n", userLevel );
	int16_t	flags = ntohs(blockData.int16at( 64 ));
	fprintf( mStackXmlFile, "\t<cantModify>%s</cantModify>\n", (flags & (1 << 15)) ? "<true />" : "<false />" );
	fprintf( mStackXmlFile, "\t<cantDelete>%s</cantDelete>\n", (flags & (1 << 14)) ? "<true />" : "<false />" );
	fprintf( mXmlFile, "\t<privateAccess>%s</privateAccess>\n", (flags & (1 << 13)) ? "<true />" : "<false />" );
	fprintf( mStackXmlFile, "\t<cantAbort>%s</cantAbort>\n", (flags & (1 << 11)) ? "<true />" : "<false />" );
	fprintf( mXmlFile, "\t<cantPeek>%s</cantPeek>\n", (flags & (1 << 10)) ? "<true />" : "<false />" );
	char		versStr[16] = { 0 };
	int32_t	version0 = blockData.int32at( 84 );
	NumVersionToStr( (unsigned char*) &version0, versStr );
	fprintf( mXmlFile, "\t<createdByVersion>HyperCard %s</createdByVersion>\n", versStr );
	int32_t	version1 = blockData.int32at( 88 );
	NumVersionToStr( (unsigned char*) &version1, versStr );
	fprintf( mXmlFile, "\t<lastCompactedVersion>HyperCard %s</lastCompactedVersion>\n", versStr );
	int32_t	version2 = blockData.int32at( 92 );
	NumVersionToStr( (unsigned char*) &version2, versStr );
	fprintf( mXmlFile, "\t<lastEditedVersion>HyperCard %s</lastEditedVersion>\n", versStr );
	int32_t	version3 = blockData.int32at( 96 );
	NumVersionToStr( (unsigned char*) &version3, versStr );
	fprintf( mXmlFile, "\t<firstEditedVersion>HyperCard %s</firstEditedVersion>\n", versStr );
	mFontTableBlockID = ntohl(blockData.int32at( 420 ));
	fprintf( mXmlFile, "\t<fontTableID>%d</fontTableID>\n", mFontTableBlockID );
	mStyleTableBlockID = ntohl(blockData.int32at( 424 ));
	fprintf( mXmlFile, "\t<styleTableID>%d</styleTableID>\n", mStyleTableBlockID );
	int16_t	height = ntohs(blockData.int16at( 428 ));
	if( height == 0 )
		height = 342;
	int16_t	width = ntohs(blockData.int16at( 430 ));
	if( width == 0 )
		width = 512;
	fprintf( mStackXmlFile, "\t<cardSize>\n\t\t<width>%d</width>\n\t\t<height>%d</height>\n\t</cardSize>\n", width, height );

	char			pattern[8] = { 0 };
	int				offs = 692;
	for( int n = 0; n < 40; n++ )
	{
		memmove( pattern, blockData.buf( offs, 8 ), 8 );
		char		fname[256] = { 0 };
		sprintf( fname, "PAT_%u.pbm", n +1 );
		picture		thePicture( 8, 8, 1, false );
		thePicture.memcopyin( pattern, 0, 8 );
		thePicture.writebitmaptopbm( fname );
		offs += 8;
		fprintf(mXmlFile,"\t<media>\n\t\t<id>%u</id>\n\t\t<type>pattern</type>\n\t\t<file>PAT_%u.pbm</file>\n\t</media>\n", n+1, n+1);
	}
	
	int x = 0, startOffs = 1524;
	fprintf( mStackXmlFile, "\t<script>" );
	for( x = startOffs; blockData[x] != 0; x++ )
	{
		char currCh = blockData[x];
		if( currCh == '<' )
			fprintf( mStackXmlFile, "&lt;" );
		else if( currCh == '>' )
			fprintf( mStackXmlFile, "&gt;" );
		else if( currCh == '&' )
			fprintf( mStackXmlFile, "&amp;" );
		else
			fprintf( mStackXmlFile, "%s", UniCharFromMacRoman(currCh) );
	}
	fprintf( mStackXmlFile, "</script>\n" );
	
	if( mProgressMessages )
		fprintf( stdout, "Progress: %d of %d\n", ++mCurrentProgress, mMaxProgress );
	
	return true;
}


bool	CStackFile::LoadStyleTable( int32_t blockID, CBuf& blockData )
{
	int32_t	vBlockSize = blockData.size();
	if( mStatusMessages )
		fprintf( stdout, "Status: Processing 'STBL' #%d %X (%d bytes)\n", blockID, blockID, vBlockSize );
	
	if( mDumpRawBlockData )
	{
		char sfn[256] = { 0 };
		snprintf( sfn, sizeof(sfn), "STBL_%d.data", blockID );
		blockData.tofile( sfn );
	}
	
	std::vector<struct CStyleEntry>	styles;
	
	size_t		currOffs = 4;
	int32_t		styleCount = ntohl(blockData.int32at( currOffs ));
	currOffs += 4;
	fprintf( mXmlFile, "\t<!-- 'STBL' #%d (%d styles) -->\n", blockID, styleCount );

	std::string	vLayerFilePath = mBasePath;
	char		vFileName[256] = { 0 };
	snprintf( vFileName, 255, "stylesheet_%d.css", blockID );
	mStyleSheetName = vFileName;
	vLayerFilePath.append( 1, '/' );
	vLayerFilePath.append( vFileName );
	
	FILE*		vStylesheetFile = fopen( vLayerFilePath.c_str(), "w" );
	
	currOffs += 2;
	int16_t	nextStyleID = ntohs(blockData.int16at( currOffs ));
	fprintf( mXmlFile, "\t<nextStyleID>%d</nextStyleID>\n", nextStyleID );
	currOffs += 2;
	currOffs += 2;
	
	for( int s = 0; s < styleCount; s++ )
	{
		CStyleEntry		style;
		
		style.mStyleID = ntohs(blockData.int16at( currOffs ));
		fprintf( vStylesheetFile, "\t\t.style%d\n\t\t{\n", style.mStyleID );
		currOffs += 2;
		currOffs += 8;
		
		style.mFontID = ntohs(blockData.int16at( currOffs ));
		if( style.mFontID != -1 )
		{
			style.mFontName = mFontTable[style.mFontID];
			fprintf( vStylesheetFile, "\t\t\tfont-family: \"%s\";\n", style.mFontName.c_str() );
		}
		currOffs += 2;
		
		int16_t	textStyleFlags = ntohs(blockData.int16at( currOffs ));
		currOffs += 2;
		
		if( textStyleFlags == 0 )
			fprintf( vStylesheetFile, "\t\t\tfont-style: normal;\n" );
		else if( textStyleFlags != -1 )	// -1 means use field style.
		{
			if( textStyleFlags & (1 << 15) )
			{
				fprintf( vStylesheetFile, "\t\t\t/* group text style */\n" );
				style.mGroup = true;
			}
			if( textStyleFlags & (1 << 14) )
			{
				fprintf( vStylesheetFile, "\t\t\tletter-spacing: 0.1em;\n" );
				style.mExtend = true;
			}
			if( textStyleFlags & (1 << 13) )
			{
				fprintf( vStylesheetFile, "\t\t\tletter-spacing: -0.1em;\n" );
				style.mCondense = true;
			}
			if( textStyleFlags & (1 << 12) )
			{
				fprintf( vStylesheetFile, "\t\t\ttext-shadow: 1px 1px #000000;\n" );
				style.mShadow = true;
			}
			if( textStyleFlags & (1 << 11) )
			{
				fprintf( vStylesheetFile, "\t\t\tcolor: white; -webkit-text-stroke-width: 1pt; -webkit-text-stroke-color: #000;\n" );
				style.mOutline = true;
			}
			if( textStyleFlags & (1 << 10) )
			{
				fprintf( vStylesheetFile, "\t\t\ttext-decoration: underline;\n" );
				style.mUnderline = true;
			}
			if( textStyleFlags & (1 << 9) )
			{
				fprintf( vStylesheetFile, "\t\t\tfont-style: italic;\n" );
				style.mItalic = true;
			}
			if( textStyleFlags & (1 << 8) )
			{
				fprintf( vStylesheetFile, "\t\t\tfont-weight: bold;\n" );
				style.mBold = true;
			}
		}
		int16_t	fontSize = ntohs(blockData.int16at( currOffs ));
		if( fontSize != -1 )
		{
			fprintf( vStylesheetFile, "\t\t\tfont-size: %dpt;\n", fontSize );
			style.mFontSize = fontSize;
		}
		currOffs += 2;
		currOffs += 8;	// 2 bytes padding?
		
		fprintf( vStylesheetFile, "\t\t}\n" );
		
		mStyles[style.mStyleID] = style;
	}
	
	fclose( vStylesheetFile );
	
	if( mProgressMessages )
		fprintf( stdout, "Progress: %d of %d\n", ++mCurrentProgress, mMaxProgress );
	
	return true;
}

bool	CStackFile::LoadFontTable( int32_t blockID, CBuf& blockData )
{
	uint32_t	vBlockSize = blockData.size();
	if( mStatusMessages )
		fprintf( stdout, "Status: Processing 'FTBL' #%d %X (%d bytes)\n", blockID, blockID, vBlockSize );

	fprintf( mXmlFile, "\t<!-- 'FTBL' #%d (%d bytes) -->\n", blockID, vBlockSize );
	int16_t	numFonts = ntohs(blockData.int16at( 6 ));
	size_t	currOffsIntoData = 8;
	currOffsIntoData += 4;	// Reserved?
	for( int n = 0; n < numFonts; n++ )
	{
		std::string		fontName;
		
		fprintf( mXmlFile, "\t<font>\n" );
		int16_t	fontID = ntohs(blockData.int16at( currOffsIntoData ));
		fprintf( mXmlFile, "\t\t<id>%d</id>\n", fontID );
		
		int x = 0, startOffs = currOffsIntoData +2;
		fprintf( mXmlFile, "\t\t<name>" );
		for( x = startOffs; blockData[x] != 0; x++ )
		{
			char currCh = blockData[x];
			if( currCh == '<' )
				fontName.append( "&lt;" );
			else if( currCh == '>' )
				fontName.append( "&gt;" );
			else if( currCh == '&' )
				fontName.append( "&amp;" );
			else
				fontName.append( (const char*) UniCharFromMacRoman(currCh) );
		}
		fprintf( mXmlFile, "%s", fontName.c_str() );
		fprintf( mXmlFile, "</name>\n" );
		
		mFontTable[fontID] = fontName;
	
		currOffsIntoData = x +1;
		currOffsIntoData += currOffsIntoData %2;	// Align on even byte.
		
		fprintf( mXmlFile, "\t</font>\n" );
	}
	
	if( mProgressMessages )
		fprintf( stdout, "Progress: %d of %d\n", ++mCurrentProgress, mMaxProgress );
	
	return true;
}


struct	CStyleRun { int16_t startOffset; int16_t styleID; };


bool	CStackFile::LoadLayerBlock( const char* vBlockType, int32_t blockID, CBuf& blockData, uint8_t inFlags )
{
	int32_t		vBlockSize = blockData.size();
	std::string	vLayerFilePath = mBasePath;
	
	bool		isCard = strcmp( "CARD", vBlockType ) == 0;
	char		vFileName[256] = { 0 };
	if( !isCard )
		snprintf( vFileName, 255, "/background_%d.xml", blockID );
	else
		snprintf( vFileName, 255, "/card_%d.xml", blockID );
	vLayerFilePath.append( vFileName );
	
	FILE*		vFile = fopen( vLayerFilePath.c_str(), "w" );
	
	if( mStatusMessages )
		fprintf( stdout, "Status: Processing '%4s' #%d %X (%d bytes)\n", vBlockType, blockID, blockID, vBlockSize );
	
	fprintf( vFile, "<?xml version=\"1.0\" encoding=\"utf-8\" ?>\n" );
	if( !isCard )
	{
		fprintf( vFile, "<!DOCTYPE background PUBLIC \"-//Apple, Inc.//DTD background V 2.0//EN\" \"\" >\n" );
		fprintf( vFile, "<background>\n" );
	}
	else
	{
		fprintf( vFile, "<!DOCTYPE card PUBLIC \"-//Apple, Inc.//DTD card V 2.0//EN\" \"\" >\n" );
		fprintf( vFile, "<card>\n" );
	}
	fprintf( vFile, "\t<id>%d</id>\n", blockID );
	
	if( mDumpRawBlockData )
	{
		char sfn[256] = { 0 };
		snprintf( sfn, sizeof(sfn), "%s_%d.data", vBlockType, blockID );
		blockData.tofile( sfn );
	}

	size_t	currOffsIntoData = 0;
	int32_t	unknownFiller = ntohl(blockData.int32at( currOffsIntoData ));
	currOffsIntoData += 4;
	fprintf( vFile, "\t<filler1>%d</filler1>\n", unknownFiller );
	int32_t	bitmapID = ntohl(blockData.int32at( currOffsIntoData ));
	currOffsIntoData += 4;
	if( bitmapID != 0 )
		fprintf( vFile, "\t<bitmap>BMAP_%u.pbm</bitmap>\n", bitmapID );
	int16_t	flags = ntohs(blockData.int16at( currOffsIntoData ));
	currOffsIntoData += 2;
	fprintf( vFile, "\t<cantDelete> %s </cantDelete>\n", (flags & (1 << 14)) ? "<true />" : "<false />" );
	fprintf( vFile, "\t<showPict> %s </showPict>\n", (flags & (1 << 13)) ? "<false />" : "<true />" );	// showPict is stored reversed.
	fprintf( vFile, "\t<dontSearch> %s </dontSearch>\n", (flags & (1 << 11)) ? "<true />" : "<false />" );
	currOffsIntoData += 14;	// Unknown data.
	int32_t	owner = -1;
	if( isCard )
	{
		owner = ntohl(blockData.int32at( currOffsIntoData ));
		fprintf( vFile, "\t<owner>%d</owner>\n", owner );
		currOffsIntoData += 4;
		
		if( inFlags & 16 )
			fprintf( vFile, "\t<marked><true /></marked>\n" );
	}

	fprintf( vFile, "\t<link rel=\"stylesheet\" type=\"text/css\" href=\"%s\" />\n", mStyleSheetName.c_str() );

	int16_t	numParts = ntohs(blockData.int16at( currOffsIntoData ));
	currOffsIntoData += 2;
	currOffsIntoData += 6;	// Unknown filler.
	int16_t	numContents = ntohs(blockData.int16at( currOffsIntoData ));
	currOffsIntoData += 2;
	currOffsIntoData += 4;	// Unknown filler.
	std::vector<int32_t>	buttonIDs;
	for( int n = 0; n < numParts; n++ )
	{
		int16_t	partLength = ntohs(blockData.int16at( currOffsIntoData ));
		
		fprintf( vFile, "\t<part>\n" );
		int16_t	partID = ntohs(blockData.int16at( currOffsIntoData +2 ));
		fprintf( vFile, "\t\t<id>%d</id>\n", partID );
		int16_t	flagsAndType = ntohs(blockData.int16at( currOffsIntoData +4 ));
		int16_t	partType = flagsAndType >> 8;
		bool	isButton = partType == 1;
		fprintf( vFile, "\t\t<type>%s</type>\n", isButton ? "button" : "field" );
		if( isButton && !isCard )
			buttonIDs.push_back( partID );
		fprintf( vFile, "\t\t<visible> %s </visible>\n", (flagsAndType & (1 << 7)) ? "<false />" : "<true />" );	// Really "hidden" flag.
		if( !isButton )
			fprintf( vFile, "\t\t<dontWrap> %s </dontWrap>\n", (flagsAndType & (1 << 5)) ? "<true />" : "<false />" );
		else
			fprintf( vFile, "\t\t<reserved5> %d </reserved5>\n", (flagsAndType & (1 << 5)) >> 5 );
		if( !isButton )
			fprintf( vFile, "\t\t<dontSearch> %s </dontSearch>\n", (flagsAndType & (1 << 4)) ? "<true />" : "<false />" );
		else
			fprintf( vFile, "\t\t<reserved4> %d </reserved4>\n", (flagsAndType & (1 << 4)) >> 4 );
		if( !isButton )
			fprintf( vFile, "\t\t<sharedText> %s </sharedText>\n", (flagsAndType & (1 << 3)) ? "<true />" : "<false />" );
		else
			fprintf( vFile, "\t\t<reserved3> %d </reserved3>\n", (flagsAndType & (1 << 3)) >> 3 );
		if( !isButton )
			fprintf( vFile, "\t\t<fixedLineHeight> %s </fixedLineHeight>\n", (flagsAndType & (1 << 2)) ? "<false />" : "<true />" );	// Really "use real line height" flag.
		else
			fprintf( vFile, "\t\t<reserved2> %d </reserved2>\n", (flagsAndType & (1 << 2)) >> 2 );
		if( !isButton )
			fprintf( vFile, "\t\t<autoTab> %s </autoTab>\n", (flagsAndType & (1 << 1)) ? "<true />" : "<false />" );
		else
			fprintf( vFile, "\t\t<reserved1> %d </reserved1>\n", (flagsAndType & (1 << 1)) >> 1 );
		if( isButton )
			fprintf( vFile, "\t\t<enabled> %s </enabled>\n", (flagsAndType & (1 << 0)) ? "<false />" : "<true />" );	// Same as lockText on fields. Really "disabled" flag.
		else
			fprintf( vFile, "\t\t<lockText> %s </lockText>\n", (flagsAndType & (1 << 0)) ? "<true />" : "<false />" );	// Same as enabled on buttons.
		fprintf( vFile, "\t\t<rect>\n\t\t\t<left>%d</left>\n\t\t\t<top>%d</top>\n\t\t\t<right>%d</right>\n\t\t\t<bottom>%d</bottom>\n\t\t</rect>\n",
					ntohs(blockData.int16at( currOffsIntoData +8 )),
					ntohs(blockData.int16at( currOffsIntoData +6 )),
					ntohs(blockData.int16at( currOffsIntoData +12 )),
					ntohs(blockData.int16at( currOffsIntoData +10 )) );
		int16_t	moreFlags = ntohs(blockData.int16at( currOffsIntoData +14 ));
		int8_t	styleFromLowNibble = moreFlags & 15;
		const char*		styleStr = "unknown";
		if( isButton )
		{
			switch( styleFromLowNibble )
			{
				case 0:
					styleStr = "transparent";
					break;
				case 1:
					styleStr = "opaque";
					break;
				case 2:
					styleStr = "rectangle";
					break;
				case 3:
					styleStr = "roundrect";
					break;
				case 4:
					styleStr = "shadow";
					break;
				case 5:
					styleStr = "checkbox";
					break;
				case 6:
					styleStr = "radiobutton";
					break;
				case 8:
					styleStr = "standard";
					break;
				case 9:
					styleStr = "default";
					break;
				case 10:
					styleStr = "oval";
					break;
				case 11:
					styleStr = "popup";
					break;
			}
		}
		else
		{
			switch( styleFromLowNibble )
			{
				case 0:
					styleStr = "transparent";
					break;
				case 1:
					styleStr = "opaque";
					break;
				case 2:
					styleStr = "rectangle";
					break;
				case 4:
					styleStr = "shadow";
					break;
				case 7:
					styleStr = "scrolling";
					break;
			}
		}
		fprintf( vFile, "\t\t<style>%s</style>\n", styleStr );
		moreFlags = moreFlags >> 8;
		int8_t	family = moreFlags & 15;
		if( isButton )
			fprintf( vFile, "\t\t<showName> %s </showName>\n", (moreFlags & (1 << 7)) ? "<true />" : "<false />" );
		else
			fprintf( vFile, "\t\t<autoSelect> %s </autoSelect>\n", (moreFlags & (1 << 7)) ? "<true />" : "<false />" );
		if( isButton )
			fprintf( vFile, "\t\t<highlight> %s </highlight>\n", (moreFlags & (1 << 6)) ? "<true />" : "<false />" );
		else
			fprintf( vFile, "\t\t<showLines> %s </showLines>\n", (moreFlags & (1 << 6)) ? "<true />" : "<false />" );
		if( !isButton )
			fprintf( vFile, "\t\t<wideMargins> %s </wideMargins>\n", (moreFlags & (1 << 5)) ? "<true />" : "<false />" );
		else
			fprintf( vFile, "\t\t<autoHighlight> %s </autoHighlight>\n", (moreFlags & (1 << 5) || family != 0) ? "<true />" : "<false />" );
		if( isButton )
			fprintf( vFile, "\t\t<sharedHighlight> %s </sharedHighlight>\n", (moreFlags & (1 << 4)) ? "<false />" : "<true />" );
		else
			fprintf( vFile, "\t\t<multipleLines> %s </multipleLines>\n", (moreFlags & (1 << 4)) ? "<true />" : "<false />" );
		if( isButton )
			fprintf( vFile, "\t\t<family>%d</family>\n", family );
		else
			fprintf( vFile, "\t\t<reservedFamily> %d </reservedFamily>\n", family );
		
		// titleWidth & iconID are list fields' lastSelectedLine and firstSelectedLine
		// 	We generate a list containing each selected line so users of the file
		//	format can add multiple selection easily.
		int16_t	titleWidth = ntohs(blockData.int16at( currOffsIntoData +16 ));
		int16_t	iconID = ntohs(blockData.int16at( currOffsIntoData +18 ));
		if( !isButton && iconID > 0 )
		{
			if( titleWidth <= 0 )
				titleWidth = iconID;
			
			fprintf( vFile, "\t\t<selectedLines>\n" );
			for( int d = iconID; d <= titleWidth; d++ )
				fprintf( vFile, "\t\t\t<integer>%d</integer>\n", d );
			fprintf( vFile, "\t\t</selectedLines>\n" );
		}
		else if( isButton && styleFromLowNibble == 11 )	// Popup buttons use icon ID for selected line:
		{
			fprintf( vFile, "\t\t<titleWidth>%d</titleWidth>\n", titleWidth );
			if( iconID != 0 )
			{
				fprintf( vFile, "\t\t<selectedLines>\n" );
				fprintf( vFile, "\t\t\t<integer>%d</integer>\n", iconID );
				fprintf( vFile, "\t\t</selectedLines>\n" );
			}
		}
		else
		{
			fprintf( vFile, "\t\t<titleWidth>%d</titleWidth>\n", titleWidth );
			fprintf( vFile, "\t\t<icon>%d</icon>\n", iconID );
		}
		int16_t	textAlign = ntohs(blockData.int16at( currOffsIntoData +20 ));
		const char*		textAlignStr = "unknown";
		switch( textAlign )
		{
			case 0:
				textAlignStr = "left";
				break;
			case 1:
				textAlignStr = "center";
				break;
			case -1:
				textAlignStr = "right";
				break;
			case -2:
				textAlignStr = "forceLeft";
				break;
		}
		fprintf( vFile, "\t\t<textAlign>%s</textAlign>\n", textAlignStr );
		int16_t	textFontID = ntohs(blockData.int16at( currOffsIntoData +22 ));
		fprintf( vFile, "\t\t<font>%s</font>\n", mFontTable[textFontID].c_str() );
		int16_t	textSize = ntohs(blockData.int16at( currOffsIntoData +24 ));
		fprintf( vFile, "\t\t<textSize>%d</textSize>\n", textSize );
		int16_t	textStyleFlags = ntohs(blockData.int16at( currOffsIntoData +26 ));
		if( textStyleFlags & (1 << 15) )
			fprintf( vFile, "\t\t<textStyle>group</textStyle>\n" );
		if( textStyleFlags & (1 << 14) )
			fprintf( vFile, "\t\t<textStyle>extend</textStyle>\n" );
		if( textStyleFlags & (1 << 13) )
			fprintf( vFile, "\t\t<textStyle>condense</textStyle>\n" );
		if( textStyleFlags & (1 << 12) )
			fprintf( vFile, "\t\t<textStyle>shadow</textStyle>\n" );
		if( textStyleFlags & (1 << 11) )
			fprintf( vFile, "\t\t<textStyle>outline</textStyle>\n" );
		if( textStyleFlags & (1 << 10) )
			fprintf( vFile, "\t\t<textStyle>underline</textStyle>\n" );
		if( textStyleFlags & (1 << 9) )
			fprintf( vFile, "\t\t<textStyle>italic</textStyle>\n" );
		if( textStyleFlags & (1 << 8) )
			fprintf( vFile, "\t\t<textStyle>bold</textStyle>\n" );
		if( textStyleFlags == 0 )
			fprintf( vFile, "\t\t<textStyle>plain</textStyle>\n" );
		
		int16_t	textHeight = ntohs(blockData.int16at( currOffsIntoData +28 ));
		if( !isButton )
			fprintf( vFile, "\t\t<textHeight>%d</textHeight>\n", textHeight );
		
		int x = 0, startOffs = currOffsIntoData +30;
		fprintf( vFile, "\t\t<name>" );
		for( x = startOffs; blockData[x] != 0; x++ )
		{
			char currCh = blockData[x];
			if( currCh == '<' )
				fprintf( vFile, "&lt;" );
			else if( currCh == '>' )
				fprintf( vFile, "&gt;" );
			else if( currCh == '&' )
				fprintf( vFile, "&amp;" );
			else
				fprintf( vFile, "%s", UniCharFromMacRoman(currCh) );
		}
		fprintf( vFile, "</name>\n" );
		
		startOffs = x +2;
		fprintf( vFile, "\t\t<script>" );
		for( x = startOffs; blockData[x] != 0; x++ )
		{
			char currCh = blockData[x];
			if( currCh == '<' )
				fprintf( vFile, "&lt;" );
			else if( currCh == '>' )
				fprintf( vFile, "&gt;" );
			else if( currCh == '&' )
				fprintf( vFile, "&amp;" );
			else
				fprintf( vFile, "%s", UniCharFromMacRoman(currCh) );
		}
		fprintf( vFile, "</script>\n" );
		
		fprintf( vFile, "\t</part>\n" );
		
		currOffsIntoData += partLength;
		currOffsIntoData += (currOffsIntoData % 2);	// Align on even byte.
	}
	
	if( !isCard )
		mButtonIDsPerBg[blockID] = buttonIDs;

	for( int n = 0; n < numContents; n++ )
	{
		/*
			Contents are a complicated thing. Essentially, they're a storage for card-specific data
			used by background objects, though for simplicity, card parts also use contents to store
			that same data. There are a few specialties, though:
			
			If the contents of a button start with a number > 32767, the number -32768 is the legth
			of style data (including this length information) at the start of the content text, and
			the actual text follows afterwards.
			
			Button contents can only be the same for all cards on a background. However, if there is
			a contents entry for a background button on a card, it contains a "1" as plain text if
			the button is highlighted and sharedHighlight is FALSE.
		*/
		
		int16_t		partID = ntohs(blockData.int16at( currOffsIntoData ));
		int16_t		partLength = ntohs(blockData.int16at( currOffsIntoData +2 ));
		bool		isBgButtonContents = false;
		
		fprintf( vFile, "\t<content>\n" );
		
		CBuf		theText, theStyles;
		if( partID < 0 )	// It's a card part's contents:
		{
			partID = -partID;
			fprintf( vFile, "\t\t<layer>card</layer>\n" );
			fprintf( vFile, "\t\t<id>%d</id>\n", partID );
			
			uint16_t	stylesLength = ntohs(blockData.uint16at( currOffsIntoData +4 ));
			if( stylesLength > 32767 )
			{
				stylesLength = stylesLength -32768;
				theStyles.resize( stylesLength -2 );
				theStyles.memcpy( 0, blockData, currOffsIntoData +6, stylesLength -2 );
			}
			else
				stylesLength = 0;
			theText.resize( partLength -stylesLength +1 );
			theText.memcpy( 0, blockData, currOffsIntoData +4 +stylesLength, partLength -stylesLength );
			theText[theText.size()-1] = 0;
			//theText.debug_print();
		}
		else	// It's a bg part's contents:
		{
			if( blockID == 9428 && partID == 20 )
				printf("\n");	// Help debug missing characters in Power Tools.
			
			fprintf( vFile, "\t\t<layer>background</layer>\n" );
			fprintf( vFile, "\t\t<id>%d</id>\n", partID );
			
			uint16_t	stylesLength = ntohs(blockData.uint16at( currOffsIntoData +4 ));
			if( stylesLength > 32767 )
			{
				stylesLength = stylesLength -32768;
				theStyles.resize( stylesLength -2 );
				theStyles.memcpy( 0, blockData, currOffsIntoData +6, stylesLength -2 );
			}
			else
				stylesLength = 0;
			theText.resize( partLength -stylesLength +1 );
			theText.memcpy( 0, blockData, currOffsIntoData +4 +stylesLength,
								partLength -stylesLength );
			theText[theText.size()-1] = 0;
			//theText.debug_print();
			
			if( owner != -1 )
			{
				std::vector<int32_t>&	buttonIDs = mButtonIDsPerBg[owner];
				for( size_t x = 0; x < buttonIDs.size(); x++ )
				{
					if( buttonIDs[x] == partID )
					{
						isBgButtonContents = true;
						break;
					}
				}
			}
		}
		
		if( !isBgButtonContents )	// Bg buttons have no per-card contents in HC.
		{
			std::vector<CStyleRun>		styleRuns;
			if( theStyles.size() > 0 )
			{
				for( size_t x = 0; x < theStyles.size(); )
				{
					int16_t	startOffset = ntohs(theStyles.int16at( x ));
					x += sizeof(int16_t);
					int16_t	styleID = ntohs(theStyles.int16at( x ));
					x += sizeof(int16_t);
					
					styleRuns.push_back( (CStyleRun){ startOffset, styleID } );
				}
			}
			int16_t		currStyleID = -1;
			size_t		currOffset = 1;
			fprintf( vFile, "\t\t<text>" );
			size_t		numChars = theText.size();
			bool		currentlyGroup = false;
			
			for( auto currRun : styleRuns )
			{
				if( currOffset < currRun.startOffset )	// Characters before this style run?
				{	// Write them all out.
					for( ; currOffset < currRun.startOffset; currOffset++ )
					{
						char currCh = theText[currOffset];
						if( currCh == '<' )
							fprintf( vFile, "&lt;" );
						else if( currCh == '>' )
							fprintf( vFile, "&gt;" );
						else if( currCh == '&' )
							fprintf( vFile, "&amp;" );
						else
							fprintf( vFile, "%s", UniCharFromMacRoman(currCh) );
					}
				}
				if( currentlyGroup )
				{
					fprintf( vFile, "</a>" );
					currentlyGroup = false;
				}
				if( currStyleID >= 0 )	// If this isn't our first style run, close previous run:
					fprintf( vFile, "</span>" );
				currStyleID = currRun.styleID;
				fprintf( vFile, "<span class=\"style%d\">", currStyleID );
				if( mStyles[currStyleID].mGroup )
				{
					fprintf( vFile, "<a href=\"#\" class=\"group\">" );
					currentlyGroup = true;
				}
			}
			for( ; currOffset < numChars; currOffset++ )
			{
				char currCh = theText[currOffset];
				if( currCh == '<' )
					fprintf( vFile, "&lt;" );
				else if( currCh == '>' )
					fprintf( vFile, "&gt;" );
				else if( currCh == '&' )
					fprintf( vFile, "&amp;" );
				else
					fprintf( vFile, "%s", UniCharFromMacRoman(currCh) );
			}
			if( currentlyGroup )
			{
				fprintf( vFile, "</a>" );
				currentlyGroup = false;
			}
			if( currStyleID >= 0 )	// If we had any style runs before this text, close it now:
				fprintf( vFile, "</span>" );
			fprintf( vFile, "</text>\n" );
		}
		else	// Bg button? May have highlight on the card:
		{
			if( theText.size() == 3 && theText[0] == 0 && theText[1] == '1' && theText[2] == 0 )	// If "shared Highlight" is FALSE, a button's highlight is stored as the content string "1".
				fprintf( vFile, "\t\t<highlight> <true /> </highlight>\n" );
		}
		
		currOffsIntoData += partLength +4 +(partLength % 2);	// Align on even byte.
		
		fprintf( vFile, "\t</content>\n" );
	}
	
	int x = 0, startOffs = currOffsIntoData;
	fprintf( vFile, "\t<name>" );
	for( x = startOffs; blockData[x] != 0; x++ )
	{
		char currCh = blockData[x];
		if( currCh == '<' )
			fprintf( vFile, "&lt;" );
		else if( currCh == '>' )
			fprintf( vFile, "&gt;" );
		else if( currCh == '&' )
			fprintf( vFile, "&amp;" );
		else
			fprintf( vFile, "%s", UniCharFromMacRoman(currCh) );
	}
	fprintf( vFile, "</name>\n" );

	if( !isCard )
		fprintf( mStackXmlFile, "\t<background id=\"%d\" file=\"%s\" name=\"", blockID, vFileName +1 );
	else
		fprintf( mStackXmlFile, "\t<card id=\"%d\" file=\"%s\" marked=\"%s\" name=\"", blockID, vFileName +1, ((inFlags & 16) ? "true" : "false") );
	
	for( x = startOffs; blockData[x] != 0; x++ )
	{
		char currCh = blockData[x];
		if( currCh == '"' )
			fprintf( mStackXmlFile, "%%22" );
		else if( currCh == '\n' )
			fprintf( mStackXmlFile, "%%0A;" );
		else if( currCh == '\r' )
			fprintf( mStackXmlFile, "%%0D" );
		else
			fprintf( mStackXmlFile, "%s", UniCharFromMacRoman(currCh) );
	}
	
	fprintf( mStackXmlFile, "\" " );
	if( isCard )
		fprintf( mStackXmlFile, "owner=\"%d\" ", owner );
	fprintf( mStackXmlFile, "/>\n" );
	
	startOffs = x +1;
	fprintf( vFile, "\t<script>" );
	for( x = startOffs; blockData[x] != 0; x++ )
	{
		char currCh = blockData[x];
		if( currCh == '<' )
			fprintf( vFile, "&lt;" );
		else if( currCh == '>' )
			fprintf( vFile, "&gt;" );
		else if( currCh == '&' )
			fprintf( vFile, "&amp;" );
		else
			fprintf( vFile, "%s", UniCharFromMacRoman(currCh) );
	}
	fprintf( vFile, "</script>\n" );
	
	if( mProgressMessages )
		fprintf( stdout, "Progress: %d of %d\n", ++mCurrentProgress, mMaxProgress );

	// Read AddColor data:
#if MAC_CODE
	OSType		theType = (!isCard) ? 'HCbg' : 'HCcd';
	Handle		currIcon = Get1Resource( theType, blockID );
	if( currIcon && GetHandleSize(currIcon) <= 0 )
		fprintf( stdout, "Progress: %d of %d\n", ++mCurrentProgress, mMaxProgress );
	else if( currIcon && GetHandleSize(currIcon) > 0 )
	{		
		if( mStatusMessages )
			fprintf( stdout, "Status: Converting AddColor '%s' %d.\n", ((theType == 'HCbg') ? "HCbg" : "HCcd"), blockID );
		
		size_t	dataLen = GetHandleSize( currIcon );
		CBuf	theData( dataLen );
		theData.memcpy( 0, *currIcon, 0, dataLen );
		
		size_t	currOffs = 0;
		while( currOffs < dataLen )
		{
			fprintf( vFile, "\t<addcolorobject>\n" );
			int8_t	currType = theData[currOffs];
			bool	vHidden = currType & (1 << 7);
			currType &= ~(1 << 7);
			currOffs += 1;
			
			switch( currType )
			{
				case 0x01:	// Button
				{
					fprintf( vFile, "\t\t<type>button</type>\n" );
					int16_t		buttonID = 0, bevelDepth = 0;
					uint16_t	r = 0, g = 0, b = 0;
					buttonID = ntohs(theData.int16at( currOffs ));
					currOffs += 2;
					fprintf( vFile, "\t\t<id>%d</id>\n", buttonID );
					bevelDepth = ntohs(theData.int16at( currOffs ));
					currOffs += 2;
					fprintf( vFile, "\t\t<bevel>%d</bevel>\n", bevelDepth );
					fprintf( vFile, "\t\t<color>\n" );
					r = ntohs(theData.uint16at( currOffs ));
					currOffs += 2;
					fprintf( vFile, "\t\t\t<red>%d</red>\n", r );
					g = ntohs(theData.uint16at( currOffs ));
					currOffs += 2;
					fprintf( vFile, "\t\t\t<green>%d</green>\n", g );
					b = ntohs(theData.uint16at( currOffs ));
					currOffs += 2;
					fprintf( vFile, "\t\t\t<blue>%d</blue>\n", b );
					fprintf( vFile, "\t\t</color>\n" );
					break;
				}

				case 0x02:	// Field
				{
					fprintf( vFile, "\t\t<type>field</type>\n" );
					int16_t		buttonID = 0, bevelDepth = 0;
					uint16_t	r = 0, g = 0, b = 0;
					buttonID = ntohs(theData.int16at( currOffs ));
					currOffs += 2;
					fprintf( vFile, "\t\t<id>%d</id>\n", buttonID );
					bevelDepth = ntohs(theData.int16at( currOffs ));
					currOffs += 2;
					fprintf( vFile, "\t\t<bevel>%d</bevel>\n", bevelDepth );
					fprintf( vFile, "\t\t<color>\n" );
					r = ntohs(theData.uint16at( currOffs ));
					currOffs += 2;
					fprintf( vFile, "\t\t\t<red>%d</red>\n", r );
					g = ntohs(theData.uint16at( currOffs ));
					currOffs += 2;
					fprintf( vFile, "\t\t\t<green>%d</green>\n", g );
					b = ntohs(theData.uint16at( currOffs ));
					currOffs += 2;
					fprintf( vFile, "\t\t\t<blue>%d</blue>\n", b );
					fprintf( vFile, "\t\t</color>\n" );
					break;
				}

				case 0x03:	// Rectangle
				{
					fprintf( vFile, "\t\t<type>rectangle</type>\n" );
					uint16_t	rc = 0, gc = 0, bc = 0;
					int16_t		bevelDepth = 0;
					int16_t		l, t, r, b;
					fprintf( vFile, "\t\t<rect>\n" );
					t = ntohs(theData.int16at( currOffs ));
					currOffs += 2;
					fprintf( vFile, "\t\t\t<top>%d</top>\n", t );
					l = ntohs(theData.int16at( currOffs ));
					currOffs += 2;
					fprintf( vFile, "\t\t\t<left>%d</left>\n", l );
					b = ntohs(theData.int16at( currOffs ));
					currOffs += 2;
					fprintf( vFile, "\t\t\t<bottom>%d</bottom>\n", b );
					r = ntohs(theData.int16at( currOffs ));
					currOffs += 2;
					fprintf( vFile, "\t\t\t<right>%d</right>\n", r );
					fprintf( vFile, "\t\t</rect>\n" );
					bevelDepth = ntohs(theData.int16at( currOffs ));
					currOffs += 2;
					fprintf( vFile, "\t\t<bevel>%d</bevel>\n", bevelDepth );
					fprintf( vFile, "\t\t<color>\n" );
					rc = ntohs(theData.uint16at( currOffs ));
					currOffs += 2;
					fprintf( vFile, "\t\t\t<red>%d</red>\n", rc );
					gc = ntohs(theData.uint16at( currOffs ));
					currOffs += 2;
					fprintf( vFile, "\t\t\t<green>%d</green>\n", gc );
					bc = ntohs(theData.uint16at( currOffs ));
					currOffs += 2;
					fprintf( vFile, "\t\t\t<blue>%d</blue>\n", bc );
					fprintf( vFile, "\t\t</color>\n" );
					break;
				}

				case 0x04:	// Picture Resource
				case 0x05:	// Picture file
				{
					fprintf( vFile, "\t\t<type>picture</type>\n" );
					int16_t		l, t, r, b;
					fprintf( vFile, "\t\t<rect>\n" );
					t = ntohs(theData.int16at( currOffs ));
					currOffs += 2;
					fprintf( vFile, "\t\t\t<top>%d</top>\n", t );
					l = ntohs(theData.int16at( currOffs ));
					currOffs += 2;
					fprintf( vFile, "\t\t\t<left>%d</left>\n", l );
					b = ntohs(theData.int16at( currOffs ));
					currOffs += 2;
					fprintf( vFile, "\t\t\t<bottom>%d</bottom>\n", b );
					r = ntohs(theData.int16at( currOffs ));
					currOffs += 2;
					fprintf( vFile, "\t\t\t<right>%d</right>\n", r );
					fprintf( vFile, "\t\t</rect>\n" );

					uint8_t	transparentFlag = theData[currOffs];
					currOffs += 1;
					fprintf( vFile, "\t\t<transparent> %s </transparent>\n", transparentFlag ? "<true />" : "<false />" );
					
					uint8_t	imageNameLen = theData[currOffs];
					currOffs += 1;
					
					if( currType == 0x05 )
						fprintf( vFile, "\t\t<file>" );
					else
						fprintf( vFile, "\t\t<name>" );
					for( int n = 0; n < imageNameLen; n++ )
					{
						char currCh = theData[currOffs++];
						if( currCh == '<' )
							fprintf( vFile, "&lt;" );
						else if( currCh == '>' )
							fprintf( vFile, "&gt;" );
						else if( currCh == '&' )
							fprintf( vFile, "&amp;" );
						else
							fprintf( vFile, "%s", UniCharFromMacRoman(currCh) );
					}
					if( currType == 0x05 )
						fprintf( vFile, "</file>\n" );
					else
						fprintf( vFile, "</name>\n" );
					currOffs++;	// Skip filler byte.
					break;
				}
				
				default:
					fprintf( vFile, "\t\t<!-- Unknown type %x, aborting '%s' resource %d. -->\n", currType, ((strcmp(vBlockType,"BKGD") == 0) ? "HCbg" : "HCcd"), blockID );
					fprintf( stderr, "Error: Unknown type %x, aborting '%s' resource %d.\n", currType, ((strcmp(vBlockType,"BKGD") == 0) ? "HCbg" : "HCcd"), blockID );
					currOffs = dataLen;	// Can't read more. Skip the rest.
					break;
			}
			fprintf( vFile, "\t\t<visible> %s </visible>\n", (vHidden ? "<false />" : "<true />") );
			fprintf( vFile, "\t</addcolorobject>\n" );
		}
		
		if( mProgressMessages )
			fprintf( stdout, "Progress: %d of %d\n", ++mCurrentProgress, mMaxProgress );
	}
#endif //MAC_CODE
		
	if( !isCard )
		fprintf( vFile, "</background>\n" );
	else
		fprintf( vFile, "</card>\n" );

	fclose( vFile );
	
	return true;
}


bool	CStackFile::LoadPageTable( int32_t blockID, CBuf& blockData )
{
	bool	success = true;
	
	if( mStatusMessages )
		fprintf( stdout, "Status: Processing 'PAGE' #%d (%lu bytes)\n", blockID, blockData.size() );

	if( mDumpRawBlockData )
	{
		char sfn[256] = { 0 };
		snprintf( sfn, sizeof(sfn), "PAGE_%d.data", blockID );
		blockData.tofile( sfn );
	}
	
	if( mCardBlockSize != -1 )
	{
		size_t		currDataOffs = 12;
		while( true )
		{
			if( !blockData.hasdata( currDataOffs, sizeof(int32_t) ) )
			{
				fprintf( stderr, "Warning: Premature end of 'PAGE' #%d (%lu bytes)\n", blockID, blockData.size() );
				break;
			}
			
			int32_t		currCardID = ntohl( blockData.int32at( currDataOffs ) );
			if( currCardID == 0 )
				break;	// End of page list. (Sentinel)
			uint8_t		cardFlags = blockData[currDataOffs +4];
			
			success = LoadLayerBlock( "CARD", currCardID, mBlockMap[CStackBlockIdentifier("CARD",currCardID)], cardFlags );
			
			currDataOffs += mCardBlockSize;
		}
	}
	else
		fprintf( stderr, "Warning: Couldn't parse 'PAGE' #%d (%lu bytes) because it preceded the page table list.\n", blockID, blockData.size() );

	if( mProgressMessages )
		fprintf( stdout, "Progress: %d of %d\n", ++mCurrentProgress, mMaxProgress );
	
	return success;
}


bool	CStackFile::LoadListBlock( CBuf& blockData )
{
	if( mStatusMessages )
		fprintf( stdout, "Status: Processing 'LIST' #%d (%lu bytes)\n", mListBlockID, blockData.size() );
	
	if( mDumpRawBlockData )
	{
		char sfn[256] = { 0 };
		snprintf( sfn, sizeof(sfn), "LIST_%d.data", mListBlockID );
		blockData.tofile( sfn );
	}

	size_t		currDataOffs = 4;
	int32_t		numPageTables = ntohl(blockData.int32at(currDataOffs));
	currDataOffs += 12;
	mCardBlockSize = ntohs(blockData.int16at(currDataOffs));
	currDataOffs += 18;
	for( int32_t r = 0; r < numPageTables; r++ )
	{
		currDataOffs += 2;
		if( !blockData.hasdata( currDataOffs, sizeof(int32_t) ) )
		{
			fprintf( stderr, "Warning: Premature end of 'LIST' #%d (%lu bytes)\n", mListBlockID, blockData.size() );
			break;
		}
		
		int32_t		currPagetableID = ntohl( blockData.int32at( currDataOffs ) );
		
		LoadPageTable( currPagetableID, mBlockMap[CStackBlockIdentifier("PAGE",currPagetableID)] );
		
		currDataOffs += 4;
	}

	if( mProgressMessages )
		fprintf( stdout, "Progress: %d of %d\n", ++mCurrentProgress, mMaxProgress );
	
	return true;
}


#if MAC_CODE
bool	CStackFile::LoadBWIcons()
{
	// Export all B/W icons:
	SInt16		numIcons = Count1Resources( 'ICON' );
	for( SInt16 x = 1; x <= numIcons; x++ )	// Get1IndResource uses 1-based indexes.
	{
		Handle		currIcon = Get1IndResource( 'ICON', x );
		ResID       theID = 0;
		ResType		theType = 0L;
		Str255		name;
		GetResInfo( currIcon, &theID, &theType, name );
		char		fname[256];
		
		if( mStatusMessages )
			fprintf( stdout, "Status: Converting 'ICON' %d.\n", theID );
		
		picture		theIcon( 32, 32, 1, false );
		theIcon.memcopyin( *currIcon, 0, 4 * 32 );
		
		theIcon.buildmaskfromsurroundings();
		
//		snprintf( fname, sizeof(fname), "ICON_%d_mask.pbm", theID );
//		theIcon.writemasktopbm( fname );
		
		snprintf( fname, sizeof(fname), "ICON_%d.pbm", theID );
		theIcon.writebitmapandmasktopbm( fname );
		
		fprintf( mXmlFile, "\t<media>\n\t\t<id>%d</id>\n"
							"\t\t<type>icon</type>\n"
							"\t\t<name>", theID );
		for( int n = 1; n <= name[0]; n++ )
		{
			char currCh = name[n];
			if( currCh == '<' )
				fprintf( mXmlFile, "&lt;" );
			else if( currCh == '>' )
				fprintf( mXmlFile, "&gt;" );
			else if( currCh == '&' )
				fprintf( mXmlFile, "&amp;" );
			else
				fprintf( mXmlFile, "%s", UniCharFromMacRoman(currCh) );
		}
		fprintf( mXmlFile, "</name>\n"
							"\t\t<file>ICON_%d.pbm</file>\n"
							"\t</media>\n", theID );
		
		if( mProgressMessages )
			fprintf( stdout, "Progress: %d of %d\n", ++mCurrentProgress, mMaxProgress );
	}
	
	return true;
}
#endif //MAC_CODE

#if MAC_CODE
bool	CStackFile::LoadPictures()
{
	// Export all PICT images:
	SInt16	numIcons = Count1Resources( 'PICT' );
	for( SInt16 x = 1; x <= numIcons; x++ )	// Get1IndResource uses 1-based indexes.
	{
		Handle		currPicture = Get1IndResource( 'PICT', x );
		ResID       theID = 0;
		ResType		theType = 0L;
		Str255		name;
		GetResInfo( currPicture, &theID, &theType, name );
		char		fname[256];
		
		if( mStatusMessages )
			fprintf( stdout, "Status: Converting 'PICT' %d.\n", theID );

		snprintf( fname, sizeof(fname), "PICT_%d.pict", theID );
		FILE*		theFile = fopen( fname, "w" );
		if( !theFile )
		{
			fprintf( stderr, "Error: Couldn't create file '%s' for 'PICT' %d.\n", fname, theID );
			return false;
		}
		
		for( int n = 0; n < 8; n++ )
			fputs( "BILL_ATKINSON_ERIC_CARLSON_KEVIN_CALHOUN_DANIEL_THOME_HYPERCARD_", theFile );	// 64 bytes repeated 8 times is a neat 512 byte header.
		fwrite( *currPicture, GetHandleSize( currPicture ), 1, theFile );
		fclose( theFile );

		fprintf( mXmlFile, "\t<media>\n\t\t<id>%d</id>\n\t\t<type>picture</type>\n\t\t<name>", theID );
		for( int n = 1; n <= name[0]; n++ )
		{
			char currCh = name[n];
			if( currCh == '<' )
				fprintf( mXmlFile, "&lt;" );
			else if( currCh == '>' )
				fprintf( mXmlFile, "&gt;" );
			else if( currCh == '&' )
				fprintf( mXmlFile, "&amp;" );
			else
				fprintf( mXmlFile, "%s", UniCharFromMacRoman(currCh) );
		}
		fprintf( mXmlFile, "</name>\n\t\t<file>PICT_%d.pict</file>\n\t</media>\n", theID );
		
		if( mProgressMessages )
			fprintf( stdout, "Progress: %d of %d\n", ++mCurrentProgress, mMaxProgress );
	}
	
	return true;
}
#endif //MAC_CODE

#if MAC_CODE
bool	CStackFile::LoadCursors()
{
	SInt16	numIcons = Count1Resources( 'CURS' );
	for( SInt16 x = 1; x <= numIcons; x++ )	// Get1IndResource uses 1-based indexes.
	{
		Handle		currIcon = Get1IndResource( 'CURS', x );
		ResID       theID = 0;
		ResType		theType = 0L;
		Str255		name;
		GetResInfo( currIcon, &theID, &theType, name );
		char		fname[256];
		
		if( mStatusMessages )
			fprintf( stdout, "Status: Converting 'CURS' %d.\n", theID );
		
		snprintf( fname, sizeof(fname), "CURS_%d.pbm", theID );
		FILE*		theFile = fopen( fname, "w" );
		if( !theFile )
		{
			fprintf( stderr, "Error: Couldn't create file '%s' for 'CURS' %d.\n", fname, theID );
			return false;
		}
		
		fputs( "P4\n16 16\n", theFile );
		fwrite( *currIcon, 2 * 16, 1, theFile );
		fputs( "\nP4\n16 16\n", theFile );
		fwrite( (*currIcon) +(2 * 16), 2 * 16, 1, theFile );
		int16_t	vertPos = * (int16_t*) ((*currIcon) +(2 * 16) +(2 * 16));
		int16_t	horzPos = * (int16_t*) ((*currIcon) +(2 * 16) +(2 * 16) +2);
		fclose( theFile );
		
		fprintf( mXmlFile, "\t<media>\n\t\t<id>%d</id>\n\t\t<type>cursor</type>\n\t\t<name>", theID );
		for( int n = 1; n <= name[0]; n++ )
		{
			char currCh = name[n];
			if( currCh == '<' )
				fprintf( mXmlFile, "&lt;" );
			else if( currCh == '>' )
				fprintf( mXmlFile, "&gt;" );
			else if( currCh == '&' )
				fprintf( mXmlFile, "&amp;" );
			else
				fprintf( mXmlFile, "%s", UniCharFromMacRoman(currCh) );
		}
		fprintf( mXmlFile, "</name>\n\t\t<file>CURS_%d.pbm</file>\n\t\t<hotspot>\n\t\t\t<left>%d</left>\n\t\t\t<top>%d</top>\n\t\t</hotspot>\n\t</media>\n", theID, horzPos, vertPos );
		
		if( mProgressMessages )
			fprintf( stdout, "Progress: %d of %d\n", ++mCurrentProgress, mMaxProgress );
	}
	
	return true;
}
#endif //MAC_CODE


#if MAC_CODE
bool	CStackFile::LoadSounds()
{
#if USE_QUICKTIME
	EnterMovies();
#endif
	SInt16	numIcons = Count1Resources( 'snd ' );
	for( SInt16 x = 1; x <= numIcons; x++ )	// Get1IndResource uses 1-based indexes.
	{
		Handle		currSound = Get1IndResource( 'snd ', x );
		ResID       theID = 0;
		ResType		theType = 0L;
		Str255		name;
		GetResInfo( currSound, &theID, &theType, name );
		char		fname[256];

		if( mStatusMessages )
			fprintf( stdout, "Status: Converting 'snd ' %d.\n", theID );

		#if USE_QUICKTIME
		snprintf( fname, sizeof(fname), "snd_%d.aiff", theID );
		
		Handle		myHandle = NewHandleClear(0);
		Handle		myDataRef = NewHandleClear( sizeof(Handle) );
		BlockMove( &myHandle, *myDataRef, sizeof(Handle) );
		
		Movie	theMovie = NewMovie( newMovieActive );
		OSErr	resErr = GetMoviesError();
		if( !theMovie || resErr != noErr )
		{
			fprintf( stderr, "Error: Error %d creating QuickTime container for 'snd ' %d.\n", (int)resErr, theID );
			return false;
		}
		
		resErr = SetMovieDefaultDataRef( theMovie, myDataRef, HandleDataHandlerSubType);
		if( !theMovie || resErr != noErr )
		{
			fprintf( stderr, "Error: Error %d specifying data reference for 'snd ' %d.\n", (int)resErr, theID );
			return false;
		}
		
		resErr = PasteHandleIntoMovie( currSound, 'snd ', theMovie, 0L, NULL );
		if( resErr != noErr )
		{
			fprintf( stderr, "Error: Error %d inserting data of 'snd ' %d into QuickTime container.\n", (int)resErr, theID );
			return false;
		}
		
		FSRef		packageRef;
		resErr = FSPathMakeRef( (UInt8*) mBasePath.c_str(), &packageRef, NULL );
		if( resErr != noErr )
		{
			fprintf( stderr, "Error: Error %d creating reference to package to export 'snd ' %d.\n", (int)resErr, theID );
			return false;
		}
		FSSpec		theSpec = { 0 };
		resErr = FSGetCatalogInfo( &packageRef, kFSCatInfoNone, NULL, NULL, &theSpec, NULL );
		if( resErr != noErr )
		{
			fprintf( stderr, "Error: Error %d converting reference to package to export 'snd ' %d.\n", (int)resErr, theID );
			return false;
		}
		theSpec.name[0] = strlen(fname);
		strcpy( ((char*)theSpec.name) +1, fname );
		resErr = ConvertMovieToFile( theMovie, NULL, &theSpec, kQTFileTypeAIFF, 'TVOD', smSystemScript, NULL, createMovieFileDeleteCurFile | movieToFileOnlyExport, NULL );
		if( resErr != noErr )
		{
			fprintf( stderr, "Error: Error %d converting 'snd ' %d to AIFF.\n", (int)resErr, theID );
			return false;
		}
		
		DisposeMovie( theMovie );
	#else	// !USE_QUICKTIME
		
		std::string		srcpath( mBasePath );
		snprintf( fname, sizeof(fname), "snd_%d.bin", theID );
		srcpath.append(1,'/');
		srcpath.append(fname);
		
		FILE	*	theFile = fopen( srcpath.c_str(), "w" );
		fwrite( *currSound, GetHandleSize(currSound), 1, theFile );
		fclose( theFile );

		std::string		fpath( mBasePath );
		snprintf( fname, sizeof(fname), "snd_%d.wav", theID );
		fpath.append(1,'/');
		fpath.append(fname);
		
		snd2wav		converter( srcpath, fpath );
		int	status = converter.convert();
		if( status != 0 )
		{
			fprintf( stderr, "Error: Error %d converting 'snd ' %d to WAVE.\n", (int)status, theID );
			return false;
		}
	#endif
		
		fprintf( mXmlFile, "\t<media>\n\t\t<id>%d</id>\n\t\t<name>", theID );
		for( int n = 1; n <= name[0]; n++ )
		{
			char currCh = name[n];
			if( currCh == '<' )
				fprintf( mXmlFile, "&lt;" );
			else if( currCh == '>' )
				fprintf( mXmlFile, "&gt;" );
			else if( currCh == '&' )
				fprintf( mXmlFile, "&amp;" );
			else
				fprintf( mXmlFile, "%s", UniCharFromMacRoman(currCh) );
		}
		fprintf( mXmlFile, "</name>\n\t\t<file>snd_%d.aiff</file>\n\t\t<type>sound</type>\n\t</media>\n", theID );
		
		if( mProgressMessages )
			fprintf( stdout, "Progress: %d of %d\n", ++mCurrentProgress, mMaxProgress );
	}
	#if USE_QUICKTIME
	ExitMovies();
	#endif
	
	return true;
}
#endif // MAC_CODE

#if MAC_CODE
bool	CStackFile::Load68000Resources()
{
	SInt16	numIcons = Count1Resources( 'XCMD' );
	for( SInt16 x = 1; x <= numIcons; x++ )	// Get1IndResource uses 1-based indexes.
	{
		Handle		currPicture = Get1IndResource( 'XCMD', x );
		ResID       theID = 0;
		ResType		theType = 0L;
		Str255		name;
		GetResInfo( currPicture, &theID, &theType, name );
		char		fname[256];
		int			nameLen = name[0];
		
		// Pascal String -> C-String:
		memmove( name, name +1, nameLen );
		name[nameLen] = 0;
		
		fprintf( stderr, "Warning: Skipping code resource 'XCMD' %d \"%s\".\n", theID, name );

		snprintf( fname, sizeof(fname), "XCMD_68k_%d_%s.data", theID, name );
		FILE*		theFile = fopen( fname, "w" );
		if( !theFile )
		{
			fprintf( stderr, "Error: Couldn't create file '%s' for 'XCMD' %d.\n", fname, theID );
			return false;
		}
		
		fwrite( *currPicture, GetHandleSize( currPicture ), 1, theFile );
		fclose( theFile );

		fprintf( mXmlFile, "\t<externalcommand type=\"command\" platform=\"mac68k\" id=\"%d\" size=\"%ld\" name=\"%s\" file=\"%s\" />\n",
							theID, GetHandleSize( currPicture ), name, fname );
		
		if( mProgressMessages )
			fprintf( stdout, "Progress: %d of %d\n", ++mCurrentProgress, mMaxProgress );
	}
	
	// Export all XFCN resources:
	numIcons = Count1Resources( 'XFCN' );
	for( SInt16 x = 1; x <= numIcons; x++ )	// Get1IndResource uses 1-based indexes.
	{
		Handle		currPicture = Get1IndResource( 'XFCN', x );
		ResID       theID = 0;
		ResType		theType = 0L;
		Str255		name;
		GetResInfo( currPicture, &theID, &theType, name );
		char		fname[256];
		int			nameLen = name[0];
		
		// Pascal String -> C-String:
		memmove( name, name +1, nameLen );
		name[nameLen] = 0;
		
		fprintf( stderr, "Warning: Skipping code resource 'XFCN' %d \"%s\".\n", theID, name );

		snprintf( fname, sizeof(fname), "XFCN_68k_%d_%s.data", theID, name );
		FILE*		theFile = fopen( fname, "w" );
		if( !theFile )
		{
			fprintf( stderr, "Error: Couldn't create file '%s' for 'XFCN' %d.\n", fname, theID );
			return false;
		}
		
		fwrite( *currPicture, GetHandleSize( currPicture ), 1, theFile );
		fclose( theFile );

		fprintf( mXmlFile, "\t<externalcommand type=\"function\" platform=\"mac68k\" id=\"%d\" size=\"%ld\" name=\"%s\" file=\"%s\" />\n",
							theID, GetHandleSize( currPicture ), name, fname );
		
		if( mProgressMessages )
			fprintf( stdout, "Progress: %d of %d\n", ++mCurrentProgress, mMaxProgress );
	}
	
	return true;
}
#endif //MAC_CODE

#if MAC_CODE
bool	CStackFile::LoadPowerPCResources()
{
	// Export all xcmd resources:
	SInt16	numIcons = Count1Resources( 'xcmd' );
	for( SInt16 x = 1; x <= numIcons; x++ )	// Get1IndResource uses 1-based indexes.
	{
		Handle		currPicture = Get1IndResource( 'xcmd', x );
		ResID       theID = 0;
		ResType		theType = 0L;
		Str255		name;
		GetResInfo( currPicture, &theID, &theType, name );
		char		fname[256];
		int			nameLen = name[0];
		
		// Pascal String -> C-String:
		memmove( name, name +1, nameLen );
		name[nameLen] = 0;
		
		fprintf( stderr, "Warning: Skipping code resource 'xcmd' %d \"%s\".\n", theID, name );

		snprintf( fname, sizeof(fname), "xcmd_ppc_%d_%s.data", theID, name );
		FILE*		theFile = fopen( fname, "w" );
		if( !theFile )
		{
			fprintf( stderr, "Error: Couldn't create file '%s' for 'xcmd' %d.\n", fname, theID );
			return false;
		}
		
		fwrite( *currPicture, GetHandleSize( currPicture ), 1, theFile );
		fclose( theFile );

		fprintf( mXmlFile, "\t<externalcommand type=\"command\" platform=\"macppc\" id=\"%d\" size=\"%ld\" name=\"%s\" file=\"%s\" />\n",
							theID, GetHandleSize( currPicture ), name, fname );
		
		if( mProgressMessages )
			fprintf( stdout, "Progress: %d of %d\n", ++mCurrentProgress, mMaxProgress );
	}
	
	// Export all XFCN resources:
	numIcons = Count1Resources( 'xfcn' );
	for( SInt16 x = 1; x <= numIcons; x++ )	// Get1IndResource uses 1-based indexes.
	{
		Handle		currPicture = Get1IndResource( 'xfcn', x );
		ResID       theID = 0;
		ResType		theType = 0L;
		Str255		name;
		GetResInfo( currPicture, &theID, &theType, name );
		char		fname[256];
		int			nameLen = name[0];
		
		// Pascal String -> C-String:
		memmove( name, name +1, nameLen );
		name[nameLen] = 0;
		
		fprintf( stderr, "Warning: Skipping code resource 'xfcn' %d \"%s\".\n", theID, name );

		snprintf( fname, sizeof(fname), "xfcn_ppc_%d_%s.data", theID, name );
		FILE*		theFile = fopen( fname, "w" );
		if( !theFile )
		{
			fprintf( stderr, "Error: Couldn't create file '%s' for 'xfcn' %d.\n", fname, theID );
			return false;
		}
		
		fwrite( *currPicture, GetHandleSize( currPicture ), 1, theFile );
		fclose( theFile );

		fprintf( mXmlFile, "\t<externalcommand type=\"function\" platform=\"macppc\" id=\"%d\" size=\"%ld\" name=\"%s\" file=\"%s\" />\n",
							theID, GetHandleSize( currPicture ), name, fname );
			
		if( mProgressMessages )
			fprintf( stdout, "Progress: %d of %d\n", ++mCurrentProgress, mMaxProgress );
	}

	return true;
}
#endif //MAC_CODE


bool	CStackFile::LoadFile( const std::string& fpath )
{
	size_t		slashPos = fpath.rfind('/');
	if( slashPos == std::string::npos )
		slashPos = 0;
	else
		slashPos += 1;
	mFileName = fpath.substr(slashPos, std::string::npos);
	std::ifstream		theFile( fpath.c_str() );
	if( !theFile.is_open() )
	{
		fprintf( stderr, "Error: Couldn't open file '%s'\n", fpath.c_str() );
		return false;
	}
	
	std::string				packagePath( fpath );
	std::string::size_type	pos = packagePath.rfind( std::string(".stak") );
	if( pos != std::string::npos )
		packagePath.resize( pos, 'X' );
	packagePath.append( ".xstk" );
	mkdir( packagePath.c_str(), 0777 );
	chdir( packagePath.c_str() );
	
	mBasePath = packagePath;
	
	packagePath.append( "/project.xml" );
	mXmlFile = fopen( packagePath.c_str(), "w" );
	if( !mXmlFile )
	{
		fprintf( stderr, "Error: Couldn't create project file at '%s'\n", packagePath.c_str() );
		return false;
	}
	
	std::string	stackPath = mBasePath;
	stackPath.append( "/stack_-1.xml" );
	mStackXmlFile = fopen( stackPath.c_str(), "w" );
	if( !mXmlFile )
	{
		fprintf( stderr, "Error: Couldn't create stack file at '%s'\n", stackPath.c_str() );
		return false;
	}
		
  #if MAC_CODE
	FSRef		fileRef;
	mResRefNum = -1;
	
	OSStatus	resErr = FSPathMakeRef( (const UInt8*) fpath.c_str(), &fileRef, NULL );
	if( resErr == noErr )
	{
		mResRefNum = FSOpenResFile( &fileRef, fsRdPerm );
		if( mResRefNum < 0 )
		{
			fprintf( stderr, "Warning: No Mac resource fork to import.\n" );
			resErr = fnfErr;
		}
	}
	else
	{
		fprintf( stderr, "Error: Error %d locating input file's resource fork.\n", (int)resErr );
		mResRefNum = -1;
	}
  #endif //MAC_CODE
	
	if( mStatusMessages )
		fprintf( stdout, "Status: Output package name is '%s'\n", packagePath.c_str() );
	
	fprintf( mXmlFile, "<?xml version=\"1.0\" encoding=\"utf-8\" ?>\n"
					"<!DOCTYPE project PUBLIC \"-//Apple, Inc.//DTD project V 2.0//EN\" \"\" >\n"
					"<project>\n" );
	
	fprintf( mStackXmlFile, "<?xml version=\"1.0\" encoding=\"utf-8\" ?>\n"
					"<!DOCTYPE stack PUBLIC \"-//Apple, Inc.//DTD stack V 2.0//EN\" \"\" >\n" );
	fprintf( mStackXmlFile, "<stack>\n\t<name>" );
	
	for( int x = 0; mFileName[x] != 0; x++ )
	{
		char currCh = mFileName[x];
		if( currCh == '<' )
			fprintf( mStackXmlFile, "&lt;" );
		else if( currCh == '>' )
			fprintf( mStackXmlFile, "&gt;" );
		else if( currCh == '&' )
			fprintf( mStackXmlFile, "&amp;" );
		else
			fputc( currCh, mStackXmlFile );	// mFileName comes from POSIX, should already be UTF8, is *definitely* not MacRoman.
	}
	fprintf( mStackXmlFile, "</name>\n" );
	
	char		vBlockType[5] = { 0 };
	u_int32_t	vBlockSize = 0;
	int32_t		vBlockID = 0;
	int			numBlocks = 0;
	
	// Read all blocks so we can random-access them. Yes, I know there are more
	//	efficient ways, but honestly, who cares?
	while( true )
	{
		theFile.read( (char*) &vBlockSize, sizeof(vBlockSize) );
		if( theFile.eof() )	// Couldn't read because we hit end of file.
			break;
		
		vBlockSize = ntohl(vBlockSize);
		theFile.read( vBlockType, 4 );
		theFile.read( (char*) &vBlockID, sizeof(vBlockID) );
		vBlockID = ntohl(vBlockID);
		
		numBlocks++;
		
		if( strcmp(vBlockType,"TAIL") == 0 && vBlockID == 0xffffffff )	// End marker block?
			break;
		else if( strcmp(vBlockType,"FREE") == 0 )	// Not a free, reusable block?
		{
			if( mStatusMessages )
				fprintf( stdout, "Status: Skipping '%4s' #%d (%d bytes)\n", vBlockType, vBlockID, vBlockSize );
			theFile.ignore( vBlockSize -12 );	// Skip rest of block data.
		}
		else
		{
			CBuf		blockData( vBlockSize -12 );
			theFile.read( blockData.buf(0,vBlockSize -12), vBlockSize -12 );
			CStackBlockIdentifier	theTypeAndID(vBlockType,vBlockID);
			mBlockMap[theTypeAndID] = blockData;
//			if( mStatusMessages )
//				fprintf( stdout, "Status: Located block %s %d - (%lu)\n", vBlockType, vBlockID, mBlockMap.size() );
		}
	}
	
	fprintf( stdout, "Status: Found %d blocks in file.\n", numBlocks);
	
	mMaxProgress = mBlockMap.size();
	mCurrentProgress = 0;
	
  #if MAC_CODE
	if( mResRefNum > 0 )
	{
		int		numResources = Count1Resources( 'ICON' ) +Count1Resources( 'PICT' ) +Count1Resources( 'CURS' )
						+Count1Resources( 'snd ' ) +Count1Resources( 'HCbg' ) +Count1Resources( 'HCcd' )
						+Count1Resources( 'XCMD' ) +Count1Resources( 'XFCN' ) +Count1Resources( 'xcmd' )
						+Count1Resources( 'xfcn' );
		mMaxProgress += numResources;
		fprintf( stdout, "Status: Found %d resources in file.\n", numResources);
	}
  #endif // MAC_CODE
	if( mProgressMessages )
		fprintf( stdout, "Progress: %d of %d\n", mCurrentProgress, mMaxProgress );
	
	// Load some "table of contents"-style blocks other blocks need to refer to:
	CBlockMap::iterator		stackItty = mBlockMap.find(CStackBlockIdentifier("STAK"));
	if( stackItty == mBlockMap.end() )
		fprintf( stderr, "Error: Couldn't find stack block." );
	bool	success = LoadStackBlock( -1, stackItty->second );	// It's always -1 in HC, but in the XML format, we may want to support other IDs.
	if( success )
		success = LoadFontTable( mFontTableBlockID, mBlockMap[CStackBlockIdentifier("FTBL",mFontTableBlockID)] );
	if( success )
		success = LoadStyleTable( mStyleTableBlockID, mBlockMap[CStackBlockIdentifier("STBL",mStyleTableBlockID)] );
	
	// Now load all backgrounds and bitmaps:
	if( success )
	{
		CBlockMap::iterator	currBlockItty = mBlockMap.begin();
		for( ; currBlockItty != mBlockMap.end(); currBlockItty++ )
		{
			if( currBlockItty->first == CStackBlockIdentifier("BMAP") )
			{
				int32_t		blockID = currBlockItty->first.mID;
				CBuf&		blockData = currBlockItty->second;
				if( mStatusMessages )
					fprintf( stdout, "Status: Processing 'BMAP' #%d %X (%lu bytes)\n", blockID, blockID, blockData.size() );
				fprintf( mXmlFile, "\t<!-- Processed 'BMAP' #%d (%lu bytes) -->\n", blockID, blockData.size() );
				
				char		fname[256];
				sprintf( fname, "BMAP_%u.pbm", blockID );
				
				if( mDecodeGraphics )
				{
					char		fname[256];
					sprintf( fname, "BMAP_%u.pbm", blockID );
					
					picture		thePicture;
					woba_decode( thePicture, blockData.buf() );
					
					thePicture.writebitmapandmasktopbm( fname );
				}
				else
				{
					char		fname[256];
					sprintf( fname, "BMAP_%u.raw", blockID );
					
					FILE*	theFile = fopen( fname, "w" );
					if( theFile )
					{
						if( blockData.size() != fwrite( blockData.buf(), 1, blockData.size(), theFile ) )
							fprintf( stderr, "Error: Writing un-decoded BMAP #%u.\n", blockID );
						fclose( theFile );
					}
					else
						fprintf( stderr, "Error: Creating file for un-decoded BMAP #%u.\n", blockID );
				}
				
				if( mProgressMessages )
					fprintf( stdout, "Progress: %d of %d\n", ++mCurrentProgress, mMaxProgress );
			}
			else if( currBlockItty->first == CStackBlockIdentifier("BKGD") )
			{
			  #if 1
				success = LoadLayerBlock( "BKGD", currBlockItty->first.mID, currBlockItty->second, 0 );
			  #else
				success = LoadBackgroundBlock( currBlockItty->first.mID, currBlockItty->second );
			  #endif
			}
			else if( currBlockItty->first != CStackBlockIdentifier("CARD")
					&& currBlockItty->first != CStackBlockIdentifier("LIST")
					&& currBlockItty->first != CStackBlockIdentifier("PAGE")
					&& currBlockItty->first != CStackBlockIdentifier("STAK")
					&& currBlockItty->first != CStackBlockIdentifier("FTBL")
					&& currBlockItty->first != CStackBlockIdentifier("STBL") )
			{
				fprintf( stderr, "Warning: Skipping block %4s #%d,\n", currBlockItty->first.mType, currBlockItty->first.mID );
				if( mProgressMessages )
					fprintf( stdout, "Progress: %d of %d\n", ++mCurrentProgress, mMaxProgress );
			}
		}
	}
	
	// Now actually load the cards, which depend on knowledge from backgrounds, stack, style table etc.:
	if( success )
		success = LoadListBlock( mBlockMap[CStackBlockIdentifier("LIST",mListBlockID)] );
		
  #if MAC_CODE
	if( mResRefNum > 0 )
	{
		LoadBWIcons();
		LoadPictures();
		LoadCursors();
		LoadSounds();
		Load68000Resources();
		LoadPowerPCResources();
		
		CloseResFile( mResRefNum );
	}
  #endif // MAC_CODE
	
	fprintf( mXmlFile, "</project>\n" );
	if( mXmlFile != stdout )
		fclose( mXmlFile );

	fprintf( mStackXmlFile, "</stack>\n" );
	if( mStackXmlFile != stdout )
		fclose( mStackXmlFile );
	
  #if MAC_CODE
	if( resErr != fnfErr && resErr != noErr )
	{
		fprintf( stderr, "Error: During conversion of Macintosh fork of stack.\n" );
		return false;
	}
  #endif // MAC_CODE
	
	return true;
}
