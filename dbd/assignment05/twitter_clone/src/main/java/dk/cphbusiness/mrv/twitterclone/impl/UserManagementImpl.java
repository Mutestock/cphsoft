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
        userCreation.serialize(jedis);
        // vvv Like, wtf is this... vvv
        return true;
    }

    @Override
    public UserOverview getUserOverview(String username) throws JedisException {
        return new UserOverview(UserCreation.deserialize(username, jedis));
    }

    // wtb decorators

    @Override
    public boolean updateUser(UserUpdate userUpdate) throws JedisException {
        UserCreation userCreation = UserCreation.deserialize(userUpdate.getUsername(), jedis);
        userCreation.update(userUpdate);
        userCreation.serialize(jedis);
        return true;
    }

    @Override
    public boolean followUser(String username, String usernameToFollow) throws JedisException {
        UserCreation userWhoFollows = UserCreation.deserialize(username, jedis);
        UserCreation userWhoGetsFollowd = UserCreation.deserialize(usernameToFollow, jedis)

        userWhoFollows.appendUsernameToFollowing(usernameToFollow);
        userWhoGetsFollowd.appenedUsernameToFollowedBy(username);

        userWhoFollows.serialize(jedis);
        userWhoGetsFollowd.serialize(jedis);
        return true;
    }

    @Override
    public boolean unfollowUser(String username, String usernameToUnfollow) throws JedisException {
        UserCreation userWhoFollows = UserCreation.deserialize(username, jedis);
        UserCreation userWhoGetsUnfollowd = UserCreation.deserialize(usernameToUnfollow, jedis)

        userWhoFollows.removeUsernameFromFollowing(usernameToUnfollow);
        userWhoGetsUnfollowd.removeUsernameFromFollowedBy(username);

        userWhoFollows.serialize(jedis);
        userWhoGetsUnfollowd.serialize(jedis);
        return true;
    }

    @Override
    public Set<String> getFollowedUsers(String username) throws JedisException {
        return UserCreation.deserialize(username, jedis).getFollowing();
    }

    @Override
    public Set<String> getUsersFollowing(String username) throws JedisException {
        UserCreation userCreation = UserCreation.deserialize(username, jedis);
        return UserCreation.deserialize(username, jedis).getFollowedBy();
    }
}
