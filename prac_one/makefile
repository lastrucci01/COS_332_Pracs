APP_NAME = prac_one
CGI_DIR = cgi-bin

debug: 
	cargo build
	cp target/debug/$(APP_NAME) $(CGI_DIR)/$(APP_NAME)
	python3 -m http.server --cgi

clean:
	cargo clean