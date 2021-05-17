package dk.cphbusiness.mrv.twitterclone.impl;

import dk.cphbusiness.mrv.twitterclone.contract.UserManagement;
import dk.cphbusiness.mrv.twitterclone.dto.UserCreation;
import dk.cphbusiness.mrv.twitterclone.dto.UserOverview;
import dk.cphbusiness.mrv.twitterclone.dto.UserUpdate;

import redis.clients.jedis.Jedis;
import redis.clients.jedis.exceptions.JedisException;

import java.util.Set;


// Why would you want these functions to return booleans???

public class UserManagementImpl implements UserManagement {

    private Jedis jedis;

    public UserManagementImpl(Jedis jedis) {
        this.jedis = jedis;
    }

    @Override
    public boolean createUser(UserCreation userCreation) throws JedisException {
        userCreation.serialize(userCreation, jedis);
        return true;
    }

    @Override
    public UserOverview getUserOverview(String username) throws JedisException {
        return new UserOverview(UserCreation.deserialize(username, jedis));
    }

    @Override
    public boolean updateUser(UserUpdate userUpdate) throws JedisException {
        UserCreation userCreation = UserCreation.deserialize(userUpdate.getUsername(), jedis);
        userCreation.update(userUpdate);
        userCreation.serialize(userCreation, jedis);
        // vvv Like, wtf is this vvv
        return true;
    }

    @Override
    public boolean followUser(String username, String usernameToFollow) throws JedisException {
        throw new UnsupportedOperationException("Not yet implemented");
    }

    @Override
    public boolean unfollowUser(String username, String usernameToUnfollow) throws JedisException {
        throw new UnsupportedOperationException("Not yet implemented");
    }

    @Override
    public Set<String> getFollowedUsers(String username) throws JedisException {
        throw new UnsupportedOperationException("Not yet implemented");
    }

    @Override
    public Set<String> getUsersFollowing(String username) throws JedisException {
        throw new UnsupportedOperationException("Not yet implemented");
    }

}
