package dev.ericksuarez.auth.server;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;

@SpringBootApplication
@EnableAuthorizationServer
public class AuthServerAppApplication {

	public static void main(String[] args) {
		SpringApplication.run(AuthServerAppApplication.class, args);
	}

}
