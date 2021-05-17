package dk.cphbusiness.mrv.twitterclone.dto;

public class UserUpdate {

    // We were taught to never use public variables unless it was absolutely necessary.
    // I don't believe that it's absolutely necessary.
    // So I'm using getters and setters
    // 

    private String username;
    private String firstname; // if null, don't update
    private String lastname; // if null, don't update
    private String birthday; // if null, don't update

    public UserUpdate(String username, String firstname, String lastname, String birthday) {
        this.username = username;
        this.firstname = firstname;
        this.lastname = lastname;
        this.birthday = birthday;
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

    public String getBirthday() {
        return this.birthday;
    }
}
