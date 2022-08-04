# cat.nvim
Failed experiment in downloading random cat image, turning it into ascii and displaying it in Neovim.

Failed as I realized far too late, that characters are much larger than pixels.
Neovim would then have to somehow shrink the font of only 1 window, while keeping
the others intact, which I don't believe is possible.

Code to download image from Google and convert to grayscale then ascii works though.
