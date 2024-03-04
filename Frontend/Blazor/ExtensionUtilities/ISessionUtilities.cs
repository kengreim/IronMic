using ISession = Blazor.Data.IronMic.ISession;

namespace Blazor.ExtensionUtilities;

public static class ISessionUtilities
{
    public static TimeSpan GetDurationSince(this ISession session, DateTime start)
    {
        var trueStart = session.StartTime > start ? session.StartTime : start;
        return session.EndTime is not null
            ? (DateTime)session.EndTime - trueStart
            : session.LastUpdated - trueStart;
    }

    public static TimeSpan GetTotalDurationSince(this IEnumerable<ISession> sessions, DateTime start)
    {
        return sessions.Aggregate(new TimeSpan(), (current, session) => current + session.GetDurationSince(start));
    }
}
