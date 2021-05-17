package dk.cphbusiness.mrv.twitterclone;

public class TimeFake {
    private long time;

    public void setCurrentTime(long t) {
        this.time = t;
    }

    public long getCurrentTimeMillis() {
        return time;
    }
}
