# Static
The files in this folder are considered *static* and won't change at
runtime. These files get embedded into the compiled binary and are served
on the `/static` path of the webserver.

This folder is organized into subfolders based on type. JavaScript in `js/`
CSS stylesheets in `css/` and images in `img/`. Please place any new files
into the correct folder.

# Images
Image files should be made as small as possible while maintaining reasonable
quality. To that end SVG and WebP filetypes are vastly preferred.
So if you have an image in another format I recommend using GIMP or similar
to resize to properly and convert it to the WebP format.