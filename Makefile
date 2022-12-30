

build:
	docker build . -t saka

sh: build
	docker run -p 8080:8080 --rm -it -v /work/ProjectsForFun/rust/slaywasm:/code  saka

run: build
	docker run -p 8080:8080 --rm -it -v /work/ProjectsForFun/rust/slaywasm:/code  saka \
		trunk serve --port 8080 --address 0.0.0.0
