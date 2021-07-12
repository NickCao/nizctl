FROM docker.io/qmkfm/qmk_configurator:latest
ENV QMK_API_URL https://example.com
RUN sed -i "s/keyboards\.qmk\.fm/raw\.githubusercontent\.com\/NickCao\/nizctl\/master\/data/g" /qmk_configurator/dist/js/*.js
