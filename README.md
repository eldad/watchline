# watchline
Watchline runs a command at an interval (similar to `watch`; without clearing the screen)

## Precise mode

To observe the behavior, compare the execution of `date --iso-8601=ns | cut -d, -f2 | cut -d+ -f1` with and without precise mode:
```
# Continuous clock skew
$ watchline 'date --iso-8601=ns | cut -d, -f2 | cut -d+ -f1'

# Maintain the same interval on average
$ watchline -p 'date --iso-8601=ns | cut -d, -f2 | cut -d+ -f1'
```
