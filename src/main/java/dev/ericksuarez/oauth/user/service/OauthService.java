package dev.ericksuarez.oauth.user.service;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.security.oauth2.config.annotation.web.configuration.EnableAuthorizationServer;


@SpringBootApplication
@EnableAuthorizationServer
public class OauthService {
    public static void main(String[] args) {
        SpringApplication.run(OauthService.class, args);
    }
}
