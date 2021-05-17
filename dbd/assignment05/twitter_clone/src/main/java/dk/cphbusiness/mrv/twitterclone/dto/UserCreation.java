package dk.cphbusiness.mrv.twitterclone.dto;

import java.util.List;
import java.util.ArrayList;

import com.google.gson.Gson;

import redis.clients.jedis.Jedis;
import redis.clients.jedis.exceptions.JedisException;

public class UserCreation {
    private String username;
    private String firstname;
    private String lastname;
    private String passwordHash;
    private String birthday;
    private int numFollowers;
    private int numFollowing;
    private List<String> followingUsers;

    // Usually you'd create a user object, and then make a DTO object from it
    // I need some values to deserialize from

    public UserCreation(String username, String firstname, String lastname, String passwordHash, String birthday) {
        this.username = username;
        this.firstname = firstname;
        this.lastname = lastname;
        this.passwordHash = passwordHash;
        this.birthday = birthday;
        this.numFollowers = 0;
        this.numFollowing = 0;
        this.followingUsers = new ArrayList<String>();
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

    public int getNumFollowers() {
        return this.numFollowers;
    }

    public int getNumFollowing() {
        return this.numFollowing;
    }

    public void update(UserUpdate userUpdate) {

        // Large setter

        String username = userUpdate.getUsername();
        String firstname = userUpdate.getFirstname();
        String lastname = userUpdate.getLastname();
        String birthday = userUpdate.getBirthday();

        if (username != null) {
            this.username = username;
        }

        if (firstname != null) {
            this.firstname = firstname;
        }

        if (lastname != null) {
            this.lastname = lastname;
        }

        if (birthday != null) {
            this.birthday = birthday;
        }
    }

    // Just merging serde and database interactions into same functions.

    public static UserCreation deserialize(String username, Jedis jedis) throws JedisException {
        Gson gson = new Gson();
        String user = jedis.get(username);
        return gson.fromJson(user, UserCreation.class);
    }

    public void serialize(UserCreation userCreation, Jedis jedis) throws JedisException{
        Gson gson = new Gson();
        String json = gson.toJson(userCreation);
        jedis.set(userCreation.getUsername(), json);
    }


    
}
