# Rust Station Images

This folder contains all tooling and documentation for the Rust Station Image Format. It sounds a lot fancier than it is. It's just a `.png` with accompanying `.yml` file to store metadata. If you are familiar with BYOND, it's the same as DMIs except the metadata is stored in a separate file, as YAML, and documented.

# Documentation

The image data itself is stored in a regular PNG file. No metadata about the icon is stored inside the PNG itself.

Metadata is stored in a YAML (`.yml`) file with the same name as the accompanying PNG. For example, if the PNG is `helloworld.png`, the YAML file would be `helloworld.yml`.

The YAML file has a `version` key that is a simple integer.

The PNG file is a spritesheet. The dimensions of the PNG are always multiples of the respective dimensions defined in the YAML content, under the root `size` element. For the `size` element: `x` is width and `y` is height.

The dimensions of the spritesheet (that is, the vertical and horizontal amount of sprites) is always attempted to be normalized so that the image is as square as possible. If this is not possible, the spritesheet will have more horizontal sprites than vertical.

Dimensions of the spritesheet are determined at runtime by dividing the image's dimensions by the sprite dimensions.

The YAML defines "states" for the spritesheet. This is similar to BYOND.

Each state maps to a range of sprites in the spritesheet. The amount of of sprites a state takes up is dependent on the following factors: amount of directions of the state and amount of frames.

States can be defined multiple times, but with different flags (see movement states in BYOND). No flags are currently defined, so this is slated for further versions. (note that v1 is directly backwards compatible, as the flags field should always be _loaded_)

States are stored in a list, under the `states` root element. Each state defines an amount of directions. The amount of directions is either `1` or `4`. Note that more amounts of directions may be added later.

As for the order in which order directions go: north south east west.

Each state has a list of lists of delays which corresponds to each direction. The delays are the delays between frames of animated sprites. The delays are counted in tenths of seconds, and floats _are_ supported.

**NOTE:** no actual amount of directions is directly stored. The amount is the length of the delays list.

## Example YAML

Note that in practice the YAML writer probably writes the most compact YAML possible to reduce file size.

```yaml
version: 1

size:
    x: 32
    y: 32

states:
    - name: hello
      dir:
          - [1, 1, 1]
          - [2, 3, 4]
          - [3, 4, 5]
          - [4, 5, 6]
```
