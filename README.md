# ASCII-ART

A simple program I made while getting started with Rust. 
It converts images to ASCII art 😁

I convert images to grayscale because I don't need color (yet). Then I quantize the pixel values based on how many ASCII characters I want to use, assigning the brighter values with bigger characters and darker values with smaller characters. And that's it. I made it to get used to Rust because I'm a noob.

### How to run:

`cargo run -- <w> <h> image_path1 image_path2...`

Where,
w: ASCII image width (u32)
h: ASCII image height (u32)
And space-separated image paths

Example:

```
cargo run -- 120 60 /src/image1.jpg /test/image2.jpg
```

### Test runs:

Original:

![tom](https://github.com/user-attachments/assets/a7290e36-948b-40e5-a18b-9d5096743d7a)

ASCII form:

![image](https://github.com/user-attachments/assets/9b661656-d003-49e7-9c41-7d3c191d9e29)



