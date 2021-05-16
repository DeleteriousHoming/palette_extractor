# Palette_extractor

## Usage
Simply type
```
cargo run /path/to/myimage.png mypalettename
```

## Todo
1. Organize resulting palette by hue: might need to transform the rgb to hsv
and order by h
2. Create structs that contain the rgb and the rgb list of colors.
3. Give the option of eliminating colors that are too similar.
4. Create release binary of the code.