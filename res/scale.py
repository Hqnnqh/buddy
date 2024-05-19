#!/usr/bin/env python3

import argparse
import os
from PIL import Image

def scale_image(input_path, output_dir, scaling_factor=4):
    # Load the image
    original_image = Image.open(input_path)
    
    # Scale the image
    scaled_image = original_image.resize((original_image.width * scaling_factor, original_image.height * scaling_factor), Image.NEAREST)
    
    # Create the output path
    input_filename = os.path.basename(input_path)
    output_path = os.path.join(output_dir, input_filename)
    
    # Save the scaled image
    scaled_image.save(output_path)
    print(f"Scaled image saved to: {output_path}")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Scale an image by a factor of 4.")
    parser.add_argument("input_path", type=str, help="The path to the input image.")
    parser.add_argument("output_dir", type=str, help="The directory to save the scaled image.")
    
    args = parser.parse_args()
    
    scale_image(args.input_path, args.output_dir)

