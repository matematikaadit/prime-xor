# Prime XOR

If you take the XOR of x and y value of a pixel coordinate, then color
the pixel black if the result is prime, you will get this pattern.

![Prime XOR Images](./prime.png)

## Generating Your Own Image

You need [rust][rustup] to compile this program.

```console
$ git clone https://github.com/matematikaadit/prime-xor.git
$ rustc -O prime-xor.rs
```

It accept the width of the image (in px) as the first argument.

```
$ ./prime-xor 600 > prime.pbm
```

It will generate a 600x600 pixels of the image above. The format is in .pbm, but
you can convert it to png, for example, using ImageMagick.

```
$ convert prime.pbm prime.png
```


## Links

- [r/math post][reddit]

## License

This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or
distribute this software, either in source code form or as a compiled
binary, for any purpose, commercial or non-commercial, and by any
means.

In jurisdictions that recognize copyright laws, the author or authors
of this software dedicate any and all copyright interest in the
software to the public domain. We make this dedication for the benefit
of the public at large and to the detriment of our heirs and
successors. We intend this dedication to be an overt act of
relinquishment in perpetuity of all present and future rights to this
software under copyright law.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.

For more information, please refer to <http://unlicense.org>


[rustup]: https://rustup.rs/
[reddit]: https://redd.it/91c35w
