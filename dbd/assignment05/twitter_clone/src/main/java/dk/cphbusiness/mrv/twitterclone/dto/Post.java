package dk.cphbusiness.mrv.twitterclone.dto;

public class Post {
    private long timestamp;
    private String message;

    public Post(long timestamp, String message) {
        this.timestamp = timestamp;
        this.message = message;
    }

    public long getTimestamp() {
        return this.timestamp;
    }

    public String getMessage() {
        return this.message;
    }
}
