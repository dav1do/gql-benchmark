FROM amazonlinux:latest 

RUN yum -y update \
 && yum -y install awslogs curl

ENV PROJECT=api
ADD docker/api/export/$PROJECT /usr/bin/$PROJECT

VOLUME ["/root/.aws"]
EXPOSE 80/tcp
ENTRYPOINT $PROJECT
