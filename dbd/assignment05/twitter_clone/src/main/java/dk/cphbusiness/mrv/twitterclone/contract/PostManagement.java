package dk.cphbusiness.mrv.twitterclone.contract;

import dk.cphbusiness.mrv.twitterclone.dto.Post;
import redis.clients.jedis.exceptions.JedisException;

import java.util.List;

public interface PostManagement {
    boolean createPost(String username, String message) throws JedisException;

    List<Post> getPosts(String username) throws JedisException;

    List<Post> getPostsBetween(String username, long timeFrom, long timeTo)  throws JedisException;
}
