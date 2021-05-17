package dk.cphbusiness.mrv.twitterclone.dto;

public class Post {
    private long timestamp;
    private String message;

    public Post(String message) {
        this.timestamp = System.currentTimeMillis();
        this.message = message;
    }

    public long getTimestamp() {
        return this.timestamp;
    }

    public String getMessage() {
        return this.message;
    }
}
