# Command-Line Help for `vid2ss`

This document contains the help content for the `vid2ss` command-line program.

**Command Overview:**

* [`vid2ss`↴](#vid2ss)

## `vid2ss`

**Usage:** `vid2ss [OPTIONS] <VID_IN> <PNG_OUT>`

###### **Arguments:**

* `<VID_IN>` — The path to the initial video file
* `<PNG_OUT>` — The path to where you want the resulting sprite-sheet to go

###### **Options:**

* `-t`, `--tile-size <TILE_SIZE>` — The dimensions of each created tile in the sprite-sheet

  Default values: `71`, `95`
* `-c`, `--columns <COLUMNS>` — The amount of columns to spread the image into
* `-f`, `--fps <FPS>` — The fps of the resulting sprite-sheet (defaults to 10)
* `-s`, `--scale <SCALE>` — The width of each frame of the outputed sprite-sheet in pixels
* `-v`, `--verbose` — Whether to use more descriptive logging

  Possible values: `true`, `false`

* `-k`, `--keep-temp` — Whether to keep the temporary GIF file used in the conversion process

  Possible values: `true`, `false`

* `-0`, `--zero` — Start indexing from zero when calculating the position of the last frame in the sprite-sheet

  Possible values: `true`, `false`

* `-m`, `--max-frames <MAX_FRAMES>` — The maximum amount of frames to be processed
* `-d`, `--md-help` — Output help as markdown

  Possible values: `true`, `false`




<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

