#!/bin/bash

# Configuration
BASE_DIR="assets/Grades Escrime"
DPI=150
QUALITY=85

# Find all PDF files recursively
find "$BASE_DIR" -type f -name "*.pdf" | while read -r pdf_file; do
    # Get the directory and filename without extension
    dir_path=$(dirname "$pdf_file")
    base_name=$(basename "$pdf_file" .pdf)
    
    # Create output directory
    output_dir="$dir_path/$base_name"
    mkdir -p "$output_dir"
    
    echo "Converting: $pdf_file -> $output_dir/page-XX.jpg"
    
    # Convert PDF to JPEGs. pdftoppm uses - to separate name and number
    # If we use "$output_dir/page" it will produce page-1.jpg, page-2.jpg...
    pdftoppm -jpeg -jpegopt quality=$QUALITY -r $DPI "$pdf_file" "$output_dir/page"
    
    # Rename pages to have leading zeros for better sorting (page-1.jpg -> page-01.jpg)
    for f in "$output_dir"/page-*.jpg; do
        # Extract the number, strip any leading dash and the .jpg extension
        num=$(basename "$f" .jpg | sed 's/page-//')
        # Remove leading zero to avoid octal interpretation in bash printf
        clean_num=$(echo "$num" | sed 's/^0*//')
        # If the number was 0 or 00, clean_num might be empty
        if [ -z "$clean_num" ]; then clean_num=0; fi
        
        new_name=$(printf "page-%02d.jpg" "$clean_num")
        
        # Only move if the name is actually different
        if [ "page-$num.jpg" != "$new_name" ]; then
            mv "$f" "$output_dir/$new_name"
        fi
    done
done

echo "Conversion complete."
