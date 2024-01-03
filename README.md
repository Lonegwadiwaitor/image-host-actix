# <div align="center">image-host-actix</div>

<div align="center">An extremely simplified image host, capable of hosting images, gifs, videos.</div><br> 

## Why is the code so bad?
This was a simple day project to take my mind off a recent passing, You could contribute if you wish! I'd love some PRs!

## Does it use compression?
Yes, it uses brotli compression to store content.

## What's "private"?
Private is a feature that allows images to have a timeout, meaning they will no longer be accessible after a certain period of time.

## How do I use this?
Fill out the config with all your desired settings, add your SSL certificate files (cert.pem & key.pem), cargo build, and off you go!

## Do you have a ShareX config for it?
I do!

```
{
  "Version": "15.0.0",
  "Name": "Image (Public)",
  "DestinationType": "ImageUploader, FileUploader",
  "RequestMethod": "POST",
  "RequestURL": "http://127.0.0.1/upload",
  "Headers": {
    "token": "your_config_token",
    "private": "false"
  },
  "Body": "MultipartFormData",
  "FileFormName": "form",
  "URL": "http://127.0.0.1/{filename}"
}
```

```
{
  "Version": "15.0.0",
  "Name": "Image (Private)",
  "DestinationType": "ImageUploader, FileUploader",
  "RequestMethod": "POST",
  "RequestURL": "http://127.0.0.1/upload",
  "Headers": {
    "token": "your_config_token",
    "private": "true"
  },
  "Body": "MultipartFormData",
  "FileFormName": "form",
  "URL": "{json:files[0].url}"
}
```
