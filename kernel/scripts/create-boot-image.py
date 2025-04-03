#!/usr/bin/env python3
"""
Pure Python script to generate a BMP boot splash image that shows a smooth gradient.
Instead of using every single 24-bit color (which makes a huge file), we sample the
full range using a step (default: 2). This produces a smaller image while still covering
the entire color spectrum in a consistent, Morton order (Z-order) layout.
"""

import struct
import itertools
import math

# Set the step for sampling each channel.
# For example, STEP=2 will produce colors from 0,2,4,...,254, plus 255.
STEP = 2

def generate_channel_values(step):
    """Generate a list of channel values from 0 to 255 inclusive with the given step."""
    values = list(range(0, 256, step))
    if values[-1] != 255:
        values.append(255)
    return values

def part1by2(x):
    """Interleave the bits of an 8-bit number with zeros.
       This function helps compute the Morton (Z-order) value."""
    x &= 0xFF  # ensure 8-bit
    x = (x | (x << 16)) & 0xFF0000FF
    x = (x | (x << 8))  & 0x0F00F00F
    x = (x | (x << 4))  & 0xC30C30C3
    x = (x | (x << 2))  & 0x49249249
    return x

def morton_order(r, g, b):
    """Compute the Morton (Z-order) for an RGB color."""
    return part1by2(r) | (part1by2(g) << 1) | (part1by2(b) << 2)

def generate_colors():
    """
    Generate all colors from the subsampled 24-bit space and return a sorted list of (R,G,B)
    tuples in Morton order.
    """
    channel_vals = generate_channel_values(STEP)
    # Generate all (R, G, B) combinations from the sampled values.
    colors = list(itertools.product(channel_vals, channel_vals, channel_vals))
    # Sort colors by Morton order (which tends to group similar colors together)
    colors.sort(key=lambda rgb: morton_order(*rgb))
    return colors

def write_bmp(filename, width, height, colors):
    """Write a 24-bit BMP file with the given width, height and list of colors."""
    # BMP rows must be a multiple of 4 bytes.
    row_padding = (4 - (width * 3) % 4) % 4
    filesize = 54 + (width * 3 + row_padding) * height  # 54-byte header + pixel data

    # BMP Header (14 bytes)
    bmp_header = (
        b'BM' +
        struct.pack('<I', filesize) +
        b'\x00\x00' + b'\x00\x00' +
        b'\x36\x00\x00\x00'
    )

    # DIB Header (40 bytes); using negative height to indicate a top-down bitmap.
    dib_header = (
        b'\x28\x00\x00\x00' +
        struct.pack('<i', width) + struct.pack('<i', -height) +
        b'\x01\x00' +
        b'\x18\x00' +
        b'\x00\x00\x00\x00' +
        struct.pack('<I', filesize - 54) +
        b'\x13\x0B\x00\x00' +
        b'\x13\x0B\x00\x00' +
        b'\x00\x00\x00\x00' +
        b'\x00\x00\x00\x00'
    )

    with open(filename, 'wb') as f:
        f.write(bmp_header)
        f.write(dib_header)

        # Write pixel data in top-down order.
        color_index = 0
        pixel_data = bytearray()

        for _ in range(height):
            row = bytearray()
            for _ in range(width):
                if color_index < len(colors):
                    # BMP expects pixel data in BGR order.
                    r, g, b = colors[color_index]
                    row += bytes([b, g, r])
                    color_index += 1
                else:
                    # Fill remaining pixels with black.
                    row += b'\x00\x00\x00'
            # Append row padding
            row += b'\x00' * row_padding
            pixel_data.extend(row)

        f.write(pixel_data)

def main():
    # Generate the subsampled colors.
    colors = generate_colors()
    total_colors = len(colors)
    print(f"Total colors in subsampled gradient: {total_colors}")

    # Compute smallest 16:9 dimensions that hold all colors.
    # We want: width = 16 * k and height = 9 * k, with (16*k * 9*k) >= total_colors.
    k = math.ceil(math.sqrt(total_colors / 144))
    width = 16 * k
    height = 9 * k
    print(f"Generating BMP with dimensions: {width}x{height} (total pixels: {width*height})")

    # Write the BMP file.
    output_filename = "color_gradient.bmp"
    write_bmp(output_filename, width, height, colors)
    print(f"Image saved as {output_filename}")

if __name__ == '__main__':
    main()
