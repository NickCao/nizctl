# nizctl
configure your niz keyboard without resorting to windows VMs

## usage
use another keyboard when invoking nizctl to avoid strange behaviors (or just tap enter fast enough)  
the generated keymap file is compatible with qmk configurator, and can be used with the container built from `data/Dockerfile`  

### nizctl pull
dump current keymap to stdout

### nizctl push
write new keymap from stdin

## supported models
- Atom66
- Micro84
- (should work on other models with minor modification)

## TODO
- [x] calibration
- [ ] macro

## credits
[niz-tools-ruby](https://github.com/cho45/niz-tools-ruby)
