## Build
cargo build

## Run
img-uploader --fs-upload-dir=/tmp/img-previews --hres=100 --vres=100 --listen=0.0.0.0:8080

## Usage

POST /img-uploader

body: application/json

```

{
    "urls": [
        "https://store-images.s-microsoft.com/image/apps.34509.13988996263393775.14fce3d5-9ee4-4b44-a5bb-e84fedf115fb.ba3770c3-d9ff-4495-bdee-f83d9a81b7aa?mode=scale&q=90&h=1080&w=1920",
        "https://www.lg.com/us/images/TV/features/TV-UHD-UM75-A-02-4K-Resolution-Desktop.jpg",
        "https://wallpaperaccess.com/full/24866.jpg"
    ]
}
```
