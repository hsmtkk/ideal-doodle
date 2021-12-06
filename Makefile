build:
	DOCKER_BUILDKIT=1 docker build --tag hsmtkk/ideal-doodle .

heroku-push:
	heroku container:push web

heroku-release: heroku-push
	heroku container:release web
