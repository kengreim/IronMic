using Microsoft.EntityFrameworkCore;

namespace Blazor.Data.IronMic;

public class IronMicDbContext : DbContext
{
    public required DbSet<Artcc> Artccs { get; init; }
    public required DbSet<Facility> Facilities { get; init; }
    public required DbSet<Position> Positions { get; set; }
    public required DbSet<ActiveControllerSession> ActiveControllerSessions { get; init; }
    public required DbSet<EndedControllerSession> EndedControllerSessions { get; init; }
    public required DbSet<ActivePositionSession> ActivePositionSessions { get; init; }
    public required DbSet<EndedPositionSession> EndedPositionSessions { get; init; }

    protected override void OnModelCreating(ModelBuilder builder)
    {
        // Artcc
        builder.Entity<Artcc>().ToTable("artccs");
        builder.Entity<Artcc>().HasKey(a => a.Id);
        builder.Entity<Artcc>().Property(a => a.Id).HasColumnName("id");
        builder.Entity<Artcc>().Property(a => a.LastUpdated).HasColumnName("last_updated");
        builder.Entity<Artcc>()
            .HasMany(a => a.Facilities)
            .WithOne(f => f.Artcc)
            .HasForeignKey(f => f.ParentArtccId)
            .IsRequired();

        // Facility
        builder.Entity<Facility>().ToTable("facilities");
        builder.Entity<Facility>().HasKey(f => f.Id);
        builder.Entity<Facility>().Property(f => f.Id).HasColumnName("id");
        builder.Entity<Facility>().Property(f => f.Name).HasColumnName("name");
        builder.Entity<Facility>().Property(f => f.Type)
            .HasColumnName("type")
            .HasConversion(
                t => t.ToString(),
                t => (FacilityType)Enum.Parse(typeof(FacilityType), t));
        builder.Entity<Facility>().Property(f => f.LastUpdated).HasColumnName("last_updated");
        builder.Entity<Facility>()
            .HasMany(f => f.ChidFacilities)
            .WithOne(f => f.ParentFacility)
            .HasForeignKey(f => f.ParentFacilityId);
        builder.Entity<Facility>()
            .HasMany(f => f.Positions)
            .WithOne(p => p.ParentFacility)
            .HasForeignKey(p => p.ParentFacilityId);

        // Positions
        builder.Entity<Position>().ToTable("positions");
        builder.Entity<Position>().HasKey(p => p.Id);
        builder.Entity<Position>().Property(p => p.Id).HasColumnName("id");
        builder.Entity<Position>().Property(p => p.Name).HasColumnName("name");
        builder.Entity<Position>().Property(p => p.RadioName).HasColumnName("radio_name");
        builder.Entity<Position>().Property(p => p.Callsign).HasColumnName("callsign");
        builder.Entity<Position>().Property(p => p.CallsignPrefix).HasColumnName("callsign_prefix");
        builder.Entity<Position>().Property(p => p.CallsignInfix).HasColumnName("callsign_infix");
        builder.Entity<Position>().Property(p => p.CallsignSuffix).HasColumnName("callsign_suffix");
        builder.Entity<Position>().Property(p => p.CallsignWithoutInfix).HasColumnName("callsign_without_infix");
        builder.Entity<Position>().Property(p => p.Frequency).HasColumnName("frequency");
        builder.Entity<Position>().Property(p => p.IsStarred).HasColumnName("starred");

        // Active Controller Session
        builder.Entity<ActiveControllerSession>().ToTable("active_controller_sessions");
        builder.Entity<ActiveControllerSession>().HasKey(c => c.Id);
        builder.Entity<ActiveControllerSession>().Property(c => c.Id).HasColumnName("id");
        builder.Entity<ActiveControllerSession>().Property(c => c.StartTime).HasColumnName("start_time");
        builder.Entity<ActiveControllerSession>().Property(c => c.EndTime).HasColumnName("end_time");
        builder.Entity<ActiveControllerSession>().Property(c => c.LastUpdated).HasColumnName("last_updated");
        builder.Entity<ActiveControllerSession>().Property(c => c.Duration).HasColumnName("duration");
        builder.Entity<ActiveControllerSession>().Property(c => c.IsActive).HasColumnName("is_active");
        builder.Entity<ActiveControllerSession>().Property(c => c.Cid).HasColumnName("cid");
        builder.Entity<ActiveControllerSession>().OwnsMany(c => c.AssociatedVnasPositions, b => b.ToJson());
        builder.Entity<ActiveControllerSession>().Property(c => c.PositionSimpleCallsign)
            .HasColumnName("position_simple_callsign");
        builder.Entity<ActiveControllerSession>().Property(c => c.ConnectedCallsign)
            .HasColumnName("connected_callsign");
        builder.Entity<ActiveControllerSession>().Property(c => c.ConnectedFrequency)
            .HasColumnName("connected_frequency");
        builder.Entity<ActiveControllerSession>().Property(c => c.PositionSessionId)
            .HasColumnName("position_session_id");
        builder.Entity<ActiveControllerSession>().Property(c => c.IsPositionSessionActive)
            .HasColumnName("position_session_is_active");
        builder.Entity<ActiveControllerSession>().Property(c => c.Id).HasColumnName("id");

        // Completed Controller Session
        builder.Entity<EndedControllerSession>().ToTable("completed_controller_sessions");
        builder.Entity<EndedControllerSession>().HasKey(c => c.Id);
        builder.Entity<EndedControllerSession>().Property(c => c.Id).HasColumnName("id");
        builder.Entity<EndedControllerSession>().Property(c => c.StartTime).HasColumnName("start_time");
        builder.Entity<EndedControllerSession>().Property(c => c.EndTime).HasColumnName("end_time");
        builder.Entity<EndedControllerSession>().Property(c => c.LastUpdated).HasColumnName("last_updated");
        builder.Entity<EndedControllerSession>().Property(c => c.Duration).HasColumnName("duration");
        builder.Entity<EndedControllerSession>().Property(c => c.IsActive).HasColumnName("is_active");
        builder.Entity<EndedControllerSession>().Property(c => c.Cid).HasColumnName("cid");
        builder.Entity<EndedControllerSession>().OwnsMany(c => c.AssociatedVnasPositions, b => b.ToJson());
        builder.Entity<EndedControllerSession>().Property(c => c.PositionSimpleCallsign)
            .HasColumnName("position_simple_callsign");
        builder.Entity<EndedControllerSession>().Property(c => c.ConnectedCallsign)
            .HasColumnName("connected_callsign");
        builder.Entity<EndedControllerSession>().Property(c => c.ConnectedFrequency)
            .HasColumnName("connected_frequency");
        builder.Entity<EndedControllerSession>().Property(c => c.PositionSessionId)
            .HasColumnName("position_session_id");
        builder.Entity<EndedControllerSession>().Property(c => c.IsPositionSessionActive)
            .HasColumnName("position_session_is_active");
        builder.Entity<ActiveControllerSession>().Property(c => c.Id).HasColumnName("id");

        // Active Position Session
        builder.Entity<ActivePositionSession>().ToTable("active_position_sessions");
        builder.Entity<ActivePositionSession>().HasKey(c => c.Id);
        builder.Entity<ActivePositionSession>().Property(c => c.Id).HasColumnName("id");
        builder.Entity<ActivePositionSession>().Property(c => c.StartTime).HasColumnName("start_time");
        builder.Entity<ActivePositionSession>().Property(c => c.EndTime).HasColumnName("end_time");
        builder.Entity<ActivePositionSession>().Property(c => c.LastUpdated).HasColumnName("last_updated");
        builder.Entity<ActivePositionSession>().Property(c => c.Duration).HasColumnName("duration");
        builder.Entity<ActivePositionSession>().Property(c => c.IsActive).HasColumnName("is_active");
        builder.Entity<ActivePositionSession>().OwnsMany(c => c.AssociatedVnasFacilities, b => b.ToJson());
        builder.Entity<ActivePositionSession>().Property(c => c.PositionSimpleCallsign)
            .HasColumnName("position_simple_callsign");

        // Ended Position Session
        builder.Entity<EndedPositionSession>().ToTable("completed_position_sessions");
        builder.Entity<EndedPositionSession>().HasKey(c => c.Id);
        builder.Entity<EndedPositionSession>().Property(c => c.Id).HasColumnName("id");
        builder.Entity<EndedPositionSession>().Property(c => c.StartTime).HasColumnName("start_time");
        builder.Entity<EndedPositionSession>().Property(c => c.EndTime).HasColumnName("end_time");
        builder.Entity<EndedPositionSession>().Property(c => c.LastUpdated).HasColumnName("last_updated");
        builder.Entity<EndedPositionSession>().Property(c => c.Duration).HasColumnName("duration");
        builder.Entity<EndedPositionSession>().Property(c => c.IsActive).HasColumnName("is_active");
        builder.Entity<EndedPositionSession>().OwnsMany(c => c.AssociatedVnasFacilities, b => b.ToJson());
        builder.Entity<EndedPositionSession>().Property(c => c.PositionSimpleCallsign)
            .HasColumnName("position_simple_callsign");
    }
}