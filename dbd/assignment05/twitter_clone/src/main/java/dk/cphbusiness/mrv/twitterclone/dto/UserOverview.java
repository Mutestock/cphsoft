package dk.cphbusiness.mrv.twitterclone.dto;

import java.util.Objects;

public class UserOverview {
    private String username;
    private String firstname;
    private String lastname;
    private int numFollowers;
    private int numFollowing;

    public UserOverview(UserCreation userCreation) {

        if (Objects.isNull(userCreation)) {
            this.username = null;
            this.firstname = null;
            this.lastname = null;
            this.numFollowers = 0;
            this.numFollowing = 0;
        } else {
            this.username = userCreation.getUsername();
            this.firstname = userCreation.getFirstname();
            this.lastname = userCreation.getLastname();
            this.numFollowers = userCreation.getNumFollowers();
            this.numFollowing = userCreation.getNumFollowing();
        }
    }

    public String getUsername() {
        return this.username;
    }

    public String getFirstname() {
        return this.firstname;
    }

    public String getLastname() {
        return this.lastname;
    }

    public Integer getNumFollowers() {
        return this.numFollowers;
    }

    public Integer getNumFollowing() {
        return this.numFollowing;
    }
}
