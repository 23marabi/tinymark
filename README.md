# TinyMark
A tiny bookmark manager.

## About
This is just a small tool I've made for myself to organize my bookmarks in a simple way,
without needing a browser.

It doesn't support icons or much of anything else right now.

A configuration file will be stored in `~/.config/tinymark/tinymark.toml`
By default the database will be stored in `~/.local/share/tinymark`

## Using
List stored bookmarks:
`tinymark list`

Add a bookmark:
`tinymark add https://example.com name`

With optional tags & description:
`tinymark add https://example.com name "test description" tags,separated,by,comma`

Delete a previously added bookmark:
`tinymark delete <link>`

You can export all the stored bookmarks to a JSON file with `tinymark export <file>`

You can then import a previously exported JSON file with `tinymark import <file>`

## JSON
This program can output in JSON format if you supply it with the `--json` argument.

Any command that returns a Bookmark will return a JSON serialized object of it.

Otherwise it will return a 'status' key of value 'success' or 'fail',
along with a 'reason' value with a full-length output.
