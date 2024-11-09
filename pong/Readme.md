git lfs is needed for large asssets now

Install git-lfs package

```bash
sudo apt install git-lfs
```

Then run

```bash 
git lfs install
```

# Generate Environment and Skybox for an HDRi

Download an HDRi image in .hdr format (from Poly Haven or similar website).

`/home/agjini/Downloads/abandoned_workshop_02_8k.hdr`

```bash

# Generate KTX2 Textures

Clone this project

```bash
git clone https://github.com/bytestring-net/bevy_skybox_cli
```

Go inside the project

```bash
# Go inside the project
cd /home/agjini/workspace/third/bevy_skybox_cli

# Generate the skybox
cargo run --release /home/agjini/Downloads/abandoned_workshop_02_8k.hdr
```