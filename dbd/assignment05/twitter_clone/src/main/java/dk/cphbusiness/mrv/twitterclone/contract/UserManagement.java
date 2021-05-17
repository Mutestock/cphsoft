package dk.cphbusiness.mrv.twitterclone.contract;

import dk.cphbusiness.mrv.twitterclone.dto.UserCreation;
import dk.cphbusiness.mrv.twitterclone.dto.UserOverview;
import dk.cphbusiness.mrv.twitterclone.dto.UserUpdate;
import redis.clients.jedis.exceptions.JedisException;

import java.util.Set;

// What's even the point of creating an interface if you're only going to implement it once?
// We're not using reflection either.

public interface UserManagement {
    boolean createUser(UserCreation userCreation) throws JedisException;

    UserOverview getUserOverview(String username) throws JedisException;

    boolean updateUser(UserUpdate userUpdate) throws JedisException;

    boolean followUser(String username, String usernameToFollow) throws JedisException;

    boolean unfollowUser(String username, String usernameToUnfollow) throws JedisException;

    Set<String> getFollowedUsers(String username) throws JedisException;

    Set<String> getUsersFollowing(String username) throws JedisException;
}
