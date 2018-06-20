//
//  ffi_toolkit.h
//  ffi-toolkit
//
//  Created by Emily Toop on 6/19/18.
//  Copyright © 2018 Emily Toop. All rights reserved.
//

#import <UIKit/UIKit.h>
#include <stdint.h>
#include <Foundation/NSObjCRuntime.h>

//! Project version number for ffi_toolkit.
FOUNDATION_EXPORT double ffi_toolkitVersionNumber;

//! Project version string for ffi_toolkit.
FOUNDATION_EXPORT const unsigned char ffi_toolkitVersionString[];

/*
 A mapping of the ErrorCode repr(C) Rust enum.
 */
typedef enum ErrorCode {
    Other,
    AuthenticationError
} ErrorCode;

/*
 A mapping of the ExternError repr(C) Rust struct.
 */
typedef struct ErrorC {
    ErrorCode code;
    char *_Nonnull message;
} ErrorC;

/*
 A mapping of the ExternResult repr(C) Rust struct.
 */
typedef struct Result {
    void* _Nullable ok; // Might be a nullptr if optional.
    ErrorC *_Nullable err;
} Result;


void destroy(void* _Nullable obj);
void destroy_c_char(char*_Nonnull value);
void destroy_raw_uuid(uuid_t* _Nonnull value);
