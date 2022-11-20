FROM maven:3.8.6-amazoncorretto-8
COPY . .
RUN mvn clean package
CMD [ "java", "-jar", "target/app.jar" ]
# CMD ["sh"]