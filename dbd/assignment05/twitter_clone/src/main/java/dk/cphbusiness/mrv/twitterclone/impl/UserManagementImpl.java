package dk.cphbusiness.mrv.twitterclone.impl;

import dk.cphbusiness.mrv.twitterclone.contract.UserManagement;
import dk.cphbusiness.mrv.twitterclone.dto.UserCreation;
import dk.cphbusiness.mrv.twitterclone.dto.UserOverview;
import dk.cphbusiness.mrv.twitterclone.dto.UserUpdate;

import redis.clients.jedis.Jedis;
import redis.clients.jedis.exceptions.JedisException;

import java.util.Set;
import java.util.Objects;

import com.google.gson.Gson;

// Why would you want these functions to return booleans???

public class UserManagementImpl implements UserManagement {

    private Jedis jedis;

    public UserManagementImpl(Jedis jedis) {
        this.jedis = jedis;
    }

    // Should return false if the user exists
    @Override
    public boolean createUser(UserCreation userCreation) throws JedisException {
        UserCreation userToCheckWhetherExists = UserCreation.deserialize(userCreation.getUsername(), jedis);

        if (Objects.isNull(userToCheckWhetherExists)) {
            userCreation.serialize(jedis);
            return true;
        }
        return false;
    }

    @Override
    public UserOverview getUserOverview(String username) throws JedisException {

        UserOverview userOverview = new UserOverview(UserCreation.deserialize(username, jedis));
        
        if(Objects.isNull(userOverview.getUsername())) {
            return null;
        }
        return userOverview;
    }

    @Override
    public boolean updateUser(UserUpdate userUpdate) throws JedisException {
        UserCreation userCreation = UserCreation.deserialize(userUpdate.getUsername(), jedis);
        if(Objects.isNull(userCreation)) {
            return false;
        }
        userCreation.update(userUpdate);
        userCreation.serialize(jedis);
        return true;
    }

    @Override
    public boolean followUser(String username, String usernameToFollow) throws JedisException {
        UserCreation userWhoFollows = UserCreation.deserialize(username, jedis);
        UserCreation userWhoGetsFollowed = UserCreation.deserialize(usernameToFollow, jedis);

        userWhoFollows.appendUsernameToFollowing(usernameToFollow);
        userWhoGetsFollowed.appenedUsernameToFollowedBy(username);

        userWhoFollows.serialize(jedis);
        userWhoGetsFollowed.serialize(jedis);
        return true;
    }

    @Override
    public boolean unfollowUser(String username, String usernameToUnfollow) throws JedisException {
        UserCreation userWhoFollows = UserCreation.deserialize(username, jedis);
        UserCreation userWhoGetsUnfollowed = UserCreation.deserialize(usernameToUnfollow, jedis);

        if (Objects.isNull(userWhoFollows) || Objects.isNull(userWhoGetsUnfollowed)) {
            return false;
        }
        
        userWhoFollows.removeUsernameFromFollowing(usernameToUnfollow);
        userWhoGetsUnfollowed.removeUsernameFromFollowedBy(username);

        userWhoFollows.serialize(jedis);
        userWhoGetsUnfollowed.serialize(jedis);
        return true;
    }

    @Override
    public Set<String> getFollowedUsers(String username) throws JedisException {        
        UserCreation userCreation = UserCreation.deserialize(username, jedis);
        if(Objects.isNull(userCreation)) {
            return null;
        }
        return userCreation.getFollowing();
    }

    @Override
    public Set<String> getUsersFollowing(String username) throws JedisException {
        UserCreation userCreation = UserCreation.deserialize(username, jedis);
        if(Objects.isNull(userCreation)) {
            return null;
        }
        return userCreation.getFollowedBy();
    }
}
