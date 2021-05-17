package dk.cphbusiness.mrv.twitterclone.impl;

import dk.cphbusiness.mrv.twitterclone.contract.PostManagement;
import dk.cphbusiness.mrv.twitterclone.dto.Post;
import redis.clients.jedis.Jedis;

import java.util.List;

public class PostManagementImpl implements PostManagement {
    private Jedis jedis;

    public PostManagementImpl(Jedis jedis) {
        this.jedis = jedis;
    }

    @Override
    public boolean createPost(String username, String message) {
        throw new UnsupportedOperationException("Not yet implemented");
    }

    @Override
    public List<Post> getPosts(String username) {
        throw new UnsupportedOperationException("Not yet implemented");
    }

    @Override
    public List<Post> getPostsBetween(String username, long timeFrom, long timeTo) {
        throw new UnsupportedOperationException("Not yet implemented");
    }
}
