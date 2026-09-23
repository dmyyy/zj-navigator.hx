(require "helix/keymaps.scm")
(require (prefix-in helix. "helix/commands.scm"))
(require (prefix-in helix.static. "helix/static.scm"))
(require "helix/editor.scm")

(provide zj-focus-left
         zj-focus-right
         zj-focus-up
         zj-focus-down)

; Try Helix split navigation first, then fall back to the matching Zellij pane move.
(define (move-focus-with-fallback helix-move direction)
  (define before (editor-focus))
  (helix-move)
  (when (equal? before (editor-focus))
    (helix.run-shell-command
     (string-append "zellij action move-focus " direction))))

(define (zj-focus-left)
  (move-focus-with-fallback helix.static.jump_view_left "left"))

(define (zj-focus-right)
  (move-focus-with-fallback helix.static.jump_view_right "right"))

(define (zj-focus-up)
  (move-focus-with-fallback helix.static.jump_view_up "up"))

(define (zj-focus-down)
  (move-focus-with-fallback helix.static.jump_view_down "down"))

(keymap (global)
 (insert ("A-h" zj-focus-left)
         ("A-j" zj-focus-down)
         ("A-k" zj-focus-up)
         ("A-l" zj-focus-right))
 (normal ("A-h" zj-focus-left)
         ("A-j" zj-focus-down)
         ("A-k" zj-focus-up)
         ("A-l" zj-focus-right))
 (select ("A-h" zj-focus-left)
         ("A-j" zj-focus-down)
         ("A-k" zj-focus-up)
         ("A-l" zj-focus-right)))
