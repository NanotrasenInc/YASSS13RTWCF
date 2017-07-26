# Documentation

The **RSI** (Rust Station Image) format is intended to be a flexible, open, and readable way <!--Insert more marketing bull that sounds good here!--> to define icons inside sprite sheets in the same vein as the BYOND `.dmi` format. An RSI is considered an "icon", and it can contain "states" which are sub sections of said master icon. These states can define custom flags, animations, and directional icons out of the box.

An RSI is a folder with a name that ends in `.rsi`, and contains a `meta.json` and one or more PNG files according to the names of states.

The image metadata (what defines states, animations, etc...) is stored in the `meta.json` file as JSON. The actual sprites are stored in sprite sheets as PNG files in the folder. Each unique state corresponds to a sprite sheet with the same name.

## JSON

The root of the JSON file contains the following values:

Key | Meaning
--- | -------
`version` | A simple integer corresponding to the RSI format version. This can be used to identify what version an RSI is and allow the implementation to correctly enable backwards compatibility modes when needed.
`size` | The dimensions of the sprites inside the RSI, stored as an associative list of `{x: ?, y: ?}`. This is _not_ the size of the PNG files that store the sprite sheet. It is used to correctly crop the individual sprites out of the sprite sheet files.
`states` | A list of _states_ that store the actual meat of the RSI, see below.

### States

A state is a container for metadata for a specific sprite sheet. They store data related to their sprite sheet like delays of animations and directions. A state has an accompanying sprite sheet.

States have Two fields that can be used to distinguish them:

Key | Meaning
--- | -------
`name` | The name of the state. Can only contain lowercase alphabetic, numerical, and some special (`_-`) characters.
`select` | A list of strings. There will be a very specific list of selectors and these can not be used arbitrarily.

States cannot have all the same identifying values. A state with different flags and same name can thus exist, while two states with the same name and no flags will be incorrect.

Other than identifiers, a state has two other fields in relation to the actual sprites as seen in game:

Key | Meaning
--- | -------
`flags` | An associative list of `key: object` for defining extra data. There is currently no usage yet.
`directions` | A number corresponding to the amount of directions a state has. This should only be a `1` or a `4`.
`delays`* | If defined, a list of lists of delays for an animated icon state. Each list in the list corresponds to a direction. The delays are floats and represent seconds.

<sup>\* Optional value.</sup>

States are always ordered alphabetically by their corresponding file name.

#### Sprite sheet

The PNG file accompanying a state is always the name of the state, with all selectors appended with plus characters, sorted alphabetically. For example, a state with name "hello" and selectors "x" and "y" would be `hello+x+y.png` on disk.

The file contains the individual states resolved with the directions and delays of the state. The size of the file is always a multiple of the RSI's `size`. Sprites are ordered from the top left to the bottom right, always going horizontally first. The amount of sprites per row or column is always made to be as equal as possible, favoring rows to be longer than columns if the amount of states is able to be divided perfectly.

### Example JSON

Note that in practice the JSON writer probably writes the most compact JSON possible to reduce file size.

```json
{
    "version": 1,

    "size": {
        "x": 32,
        "y": 32
    },
    "states": [
        {
            "name": "hello",
            "select": [],
            "flags": {},
            "directions": 4,
            "delays": [
                [1, 1, 1],
                [2, 3, 4],
                [3, 4, 5],
                [4, 5, 6]
            ]
        }
    ]
}
```

# Design Goals

* Editing an RSI must be possible without proper tooling. This means no binary metadata or metadata inside PNG files.
* It must be easily diffable on GitHub.
* It must not bloat Git history too much when changes are made (prevent large file rewrites).
