
test:
  cargo test
  embedded-runner collect

mantra:
  rm -f mantra.db 
  mantra collect
  mantra report --formats=html mantra_report.html
