-- M0RX Language Support for Neovim
vim.api.nvim_create_autocmd({"BufRead", "BufNewFile"}, {
  pattern = "*.mrx",
  callback = function()
    vim.bo.filetype = "m0rx"
  end
})

vim.api.nvim_create_autocmd("FileType", {
  pattern = "m0rx",
  callback = function()
    vim.bo.commentstring = "// %s"
    vim.bo.tabstop = 4
    vim.bo.shiftwidth = 4
    vim.bo.expandtab = true
  end
})
