using Dapper;
using Npgsql;

namespace Blazor.Data.IronMic;

public class IronMicRepository(NpgsqlConnection dbConnection)
{
    public async Task<IEnumerable<ControllerSession>> GetAllActiveConnections()
    {
        await dbConnection.OpenAsync();
        var result =
            await dbConnection.QueryAsync<ControllerSession, PositionSession, ControllerSession>(
                ControllerSessionsSelectAll("active_controller_sessions", "active_position_sessions"),
                (c, p) =>
                {
                    c.PositionSession = p;
                    return c;
                });

        await dbConnection.CloseAsync();
        return result;
    }

    public async Task<IEnumerable<ControllerSession>> GetAllConnections()
    {
        await dbConnection.OpenAsync();
        var result =
            await dbConnection.QueryAsync<ControllerSession, PositionSession, ControllerSession>(
                ControllerSessionsSelectAll("controller_sessions", "position_sessions"),
                (c, p) =>
                {
                    c.PositionSession = p;
                    return c;
                });
        await dbConnection.CloseAsync();
        return result;
    }

    public async Task<IEnumerable<PositionSession>> GetAllActivePositions()
    {
        await dbConnection.OpenAsync();
        var result =
            await dbConnection.QueryAsync<PositionSession>(PositionSessionsSelectAll("active_position_sessions"));
        await dbConnection.CloseAsync();
        return result;
    }

    public async Task<IEnumerable<PositionSession>> GetAllPositions()
    {
        await dbConnection.OpenAsync();
        var result =
            await dbConnection.QueryAsync<PositionSession>(PositionSessionsSelectAll("position_sessions"));
        await dbConnection.CloseAsync();
        return result;
    }

    private static string ControllerSessionsSelectAll(string controllersTableName, string positionsTableName)
    {
        return $"""
                select
                    c.id,
                    c.start_time as starttime,
                    c.end_time as endtime,
                    c.last_updated as lastupdated,
                    c.duration,
                    c.datafeed_first as datafeedfirstseen,
                    c.datafeed_last as datafeedlastseen,
                    c.is_active as isactive,
                    c.cid,
                    c.position_simple_callsign as positionsimplecallsign,
                    c.connected_callsign as connectedcallsign,
                    c.connected_frequency as connectedfrequency,
                    c.position_session_id as positionsessionid,
                    c.position_session_is_active as ispositionsessionactive,
                    c.is_cooling_down as iscoolingdown,
                    p.id,
                    p.start_time as starttime,
                    p.end_time as endtime,
                    p.last_updated as lastupdated,
                    p.duration,
                    p.datafeed_first as datafeedfirstseen,
                    p.datafeed_last as datafeedlastseen,
                    p.is_active as isactive,
                    p.position_simple_callsign as positionsimplecallsign,
                    p.is_cooling_down as iscoolingdown
                from {controllersTableName} c
                left join {positionsTableName} p on c.position_session_id = p.id
                """;
    }

    private static string PositionSessionsSelectAll(string tableName)
    {
        return $"""
                select
                    id,
                    start_time as starttime,
                    end_time as endtime,
                    last_updated as lastupdated,
                    duration,
                    datafeed_first as datafeedfirstseen,
                    datafeed_last as datafeedlastseen,
                    is_active as isactive,
                    position_simple_callsign as positionsimplecallsign,
                    is_cooling_down as iscoolingdown
                from {tableName}
                """;
    }
}