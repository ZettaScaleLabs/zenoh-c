//
// Copyright (c) 2023 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>

#include <assert.h>
#include <stddef.h>
#include <stdio.h>
#include <string.h>

#include "zenoh.h"

#undef NDEBUG
#include <assert.h>

void writting_through_map_by_alias() {
    // Writing
    z_owned_bytes_map_t map = z_bytes_map_new();
    z_bytes_map_insert_by_alias(&map, z_bytes_new("k1"), z_bytes_new("v1"));
    z_bytes_map_insert_by_alias(&map, z_bytes_new("k2"), z_bytes_new("v2"));
    z_attachment_t attachment = z_bytes_map_as_attachment(&map);

    // Size check
    assert(z_attachment_len(attachment) == 2);

    // Elements check
    // z_bytes_t a = z_attachment_get(attachment, z_bytes_new("k1"));
}

void writting_through_map_by_copy() {
    // Writing
    z_owned_bytes_map_t map = z_bytes_map_new();
    z_bytes_map_insert_by_copy(&map, z_bytes_new("k1"), z_bytes_new("v1"));
    z_bytes_map_insert_by_copy(&map, z_bytes_new("k2"), z_bytes_new("v2"));
    z_attachment_t attachment = z_bytes_map_as_attachment(&map);

    // Size check
    assert(z_attachment_len(attachment) == 2);
}

int main(int argc, char **argv) {
    writting_through_map_by_alias();
    writting_through_map_by_copy();
}
