package dk.cphbusiness.mrv.twitterclone.impl;

import dk.cphbusiness.mrv.twitterclone.contract.PostManagement;
import dk.cphbusiness.mrv.twitterclone.dto.Post;
import dk.cphbusiness.mrv.twitterclone.dto.UserCreation;

import redis.clients.jedis.Jedis;
import redis.clients.jedis.exceptions.JedisException;

import java.util.Objects;
import java.util.List;
import java.util.stream.Collectors;

public class PostManagementImpl implements PostManagement {
    private Jedis jedis;

    public PostManagementImpl(Jedis jedis) {
        this.jedis = jedis;
    }

    @Override
    public boolean createPost(String username, String message) throws JedisException{
        UserCreation userCreation = UserCreation.deserialize(username, jedis);
        if(Objects.isNull(userCreation)) {
            return false;
        }
        
        userCreation.appendToPosts(new Post(message));
        userCreation.serialize(jedis);
        return true;
    }

    @Override
    public List<Post> getPosts(String username) throws JedisException {
        UserCreation userCreation = UserCreation.deserialize(username, jedis);
        if(Objects.isNull(userCreation)) {
            return null;
        }

        return userCreation.getPosts();
    }

    @Override
    public List<Post> getPostsBetween(String username, long timeFrom, long timeTo) throws JedisException {
        return UserCreation.deserialize(username, jedis)
            .getPosts()
            .stream()
            .filter(post -> (post.getTimestamp() >= timeFrom && post.getTimestamp() <= timeTo))
            .collect(Collectors.toList());
    }
}
