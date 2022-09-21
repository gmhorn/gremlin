.PHONY: clean

clean:
	@echo "==> cleaning"
	@find . -type f -name '*.png' -delete
	@find . -type f -name '*.prof' -delete