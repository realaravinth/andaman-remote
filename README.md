# andaman-remote
Creates the dreaded Andaman effect

# Dependencies
- ffmepg
- [v4l2loopback](https://github.com/umlaeute/v4l2loopback)

# Install
`v4l2loop` creates loopback camera devices. So after installing it, run:

1. Load the module with <br>
`$ sudo  modprobe v4l2loopback`
2. Create a loopback device called *USB Cam* with <br>
`$ sudo modprobe v4l2loopback devices=1 
video_nr=10 card_label="USB Cam" exclusive_caps=1`
3. Place `andaman` in `/usr/bin/`

And you should be good to go

# Launch
The server binds itself to port `8000` and can be launched with
`./adman-remote`
