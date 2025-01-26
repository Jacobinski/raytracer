build:
    cargo build

run:
    rm -f image.ppm
    cargo run >> image.ppm
    open image.ppm

# This requires ImageMagick to be installed on your system.
#   $ brew install imagemagick
publish:
    magick image.ppm raytrace.png

diff:
    magick image.ppm /tmp/raytrace.png
    compare raytrace.png /tmp/raytrace.png -compose src /tmp/diff.png
    open /tmp/diff.png
