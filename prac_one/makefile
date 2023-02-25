APP_NAME = prac_one
CGI_DIR = cgi-bin

debug: 
	cargo build
	cp target/debug/$(APP_NAME) $(CGI_DIR)/$(APP_NAME)
	python3 -m http.server --cgi

release:
	cargo build
	sudo cp target/debug/${APP_NAME} /var/www/${CGI_DIR}/${APP_NAME}
	sudo cp src/style.css /var/www/style/style.css

clean:
	cargo clean