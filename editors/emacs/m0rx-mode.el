;;; m0rx-mode.el - M0RX Language Support for Emacs
(defvar m0rx-keywords
  '("if" "else" "elif" "match" "when" "loop" "while" "each"
    "break" "skip" "give" "halt" "fn" "method" "class" "trait"
    "impl" "let" "fix" "bind" "mod" "use" "async" "await"
    "spawn" "try" "catch" "panic" "model" "infer" "embed"))
(defvar m0rx-types
  '("tiny" "short" "ant" "long" "vast" "half" "dbl" "precise"
    "str" "txt" "bool" "nil" "list" "map" "set" "blob" "tensor"))
(defvar m0rx-font-lock
  `((,(regexp-opt m0rx-keywords 'words) . font-lock-keyword-face)
    (,(regexp-opt m0rx-types 'words) . font-lock-type-face)
    ("//.*$" . font-lock-comment-face)
    ("\"[^\"]*\"" . font-lock-string-face)))
(define-derived-mode m0rx-mode prog-mode "M0RX"
  (setq font-lock-defaults '(m0rx-font-lock)))
(add-to-list 'auto-mode-alist '("\\.mrx\\'" . m0rx-mode))
(provide 'm0rx-mode)
