package: example

; (import :drewc/ftw :std/format)

; (def test-ftw-http-server #f)

; (def (ensure-ftw-server! address: (address "127.0.0.1") port: (port 8389))
;  (def saddress (format "~a:~a" address port))
;  (or test-ftw-http-server
;      (let ((s (start-ftw-http-server! saddress)))
;        (set! test-ftw-http-server s)
;        s)))


(export main)
(def (main . args)
  (displayln "hello world")
  ; (ensure-ftw-server!)
  )