// using Microsoft.EntityFrameworkCore;
//
// namespace Blazor.Data.IronMic;
//
// public class IronMicDbContext(DbContextOptions<IronMicDbContext> options) : DbContext(options)
// {
//     public required DbSet<Artcc> Artccs { get; init; }
//     public required DbSet<Facility> Facilities { get; init; }
//     public required DbSet<Position> Positions { get; set; }
//
//     // public required DbSet<ActiveControllerSession> ActiveControllerSessions { get; init; }
//     // public required DbSet<EndedControllerSession> EndedControllerSessions { get; init; }
//     // public required DbSet<ActivePositionSession> ActivePositionSessions { get; init; }
//     // public required DbSet<EndedPositionSession> EndedPositionSessions { get; init; }
//
//     public required DbSet<DatafeedRecord> DatafeedRecords { get; init; }
//
//     protected override void OnModelCreating(ModelBuilder builder)
//     {
//         // Artcc
//         builder.Entity<Artcc>(entity =>
//         {
//             entity.ToTable("artccs");
//             entity.HasKey(a => a.Id);
//             entity.Property(a => a.Id).HasColumnName("id");
//             entity.Property(a => a.LastUpdated).HasColumnName("last_updated");
//             entity.HasMany(a => a.Facilities)
//                 .WithOne(f => f.Artcc)
//                 .HasForeignKey(f => f.ParentArtccId)
//                 .IsRequired();
//         });
//
//         // Facility
//         builder.Entity<Facility>(entity =>
//         {
//             entity.ToTable("facilities");
//             entity.HasKey(f => f.Id);
//             entity.Property(f => f.Id).HasColumnName("id");
//             entity.Property(f => f.Name).HasColumnName("name");
//             entity.Property(f => f.Type)
//                 .HasColumnName("type")
//                 .HasConversion(
//                     t => t.ToString(),
//                     t => (FacilityType)Enum.Parse(typeof(FacilityType), t));
//             entity.Property(f => f.LastUpdated).HasColumnName("last_updated");
//             entity.Property(e => e.ParentArtccId).HasColumnName("parent_artcc_id");
//             entity.Property(e => e.ParentFacilityId).HasColumnName("parent_facility_id");
//             entity.HasMany(f => f.ChidFacilities)
//                 .WithOne(f => f.ParentFacility)
//                 .HasForeignKey(f => f.ParentFacilityId);
//             entity.HasMany(f => f.Positions)
//                 .WithOne(p => p.ParentFacility)
//                 .HasForeignKey(p => p.ParentFacilityId);
//         });
//
//         // Position
//         builder.Entity<Position>(entity =>
//         {
//             entity.ToTable("positions");
//             entity.HasKey(p => p.Id);
//             entity.Property(p => p.Id).HasColumnName("id");
//             entity.Property(p => p.Name).HasColumnName("name");
//             entity.Property(p => p.RadioName).HasColumnName("radio_name");
//             entity.Property(p => p.Callsign).HasColumnName("callsign");
//             entity.Property(p => p.CallsignPrefix).HasColumnName("callsign_prefix");
//             entity.Property(p => p.CallsignInfix).HasColumnName("callsign_infix");
//             entity.Property(p => p.CallsignSuffix).HasColumnName("callsign_suffix");
//             entity.Property(p => p.CallsignWithoutInfix).HasColumnName("callsign_without_infix");
//             entity.Property(p => p.Frequency).HasColumnName("frequency");
//             entity.Property(p => p.IsStarred).HasColumnName("starred");
//             entity.Property(p => p.LastUpdated).HasColumnName("last_updated");
//         });
//
//         // Positions
//         builder.Entity<ActiveControllerSession>(entity =>
//         {
//             entity.ToTable("active_controller_sessions");
//             entity.HasKey(c => new { c.Id, c.IsActive });
//             entity.Property(c => c.Id).HasColumnName("id");
//             entity.Property(c => c.StartTime).HasColumnName("start_time");
//             entity.Property(c => c.EndTime).HasColumnName("end_time");
//             entity.Property(c => c.LastUpdated).HasColumnName("last_updated");
//             entity.Property(c => c.Duration).HasColumnName("duration");
//             entity.Property(c => c.DatafeedFirstSeen).HasColumnName("datafeed_first");
//             entity.Property(c => c.DatafeedLastSeen).HasColumnName("datafeed_last");
//             entity.Property(c => c.IsActive).HasColumnName("is_active");
//             entity.Property(c => c.Cid).HasColumnName("cid");
//             entity.Property(c => c.PositionSimpleCallsign).HasColumnName("position_simple_callsign");
//             entity.Property(c => c.ConnectedCallsign).HasColumnName("connected_callsign");
//             entity.Property(c => c.ConnectedFrequency).HasColumnName("connected_frequency");
//             entity.Property(c => c.PositionSessionId).HasColumnName("position_session_id");
//             entity.Property(c => c.IsPositionSessionActive).HasColumnName("position_session_is_active");
//             entity.Property(c => c.IsCoolingDown).HasColumnName("is_cooling_down");
//             // entity.HasMany(c => c.Positions).WithMany(p => p.ControllerSessions)
//             //     .UsingEntity<ControllerSessionPositionJoin>(
//             //         l => l.HasOne<Position>(c => c.))
//         });
//
//
//         //
//         // // Active Controller Session
//         // builder.Entity<ActiveControllerSession>().ToTable("active_controller_sessions");
//         // builder.Entity<ActiveControllerSession>().HasKey(c => new { c.Id, c.IsActive });
//         // builder.Entity<ActiveControllerSession>().Property(c => c.Id).HasColumnName("id");
//         // builder.Entity<ActiveControllerSession>().Property(c => c.StartTime).HasColumnName("start_time");
//         // builder.Entity<ActiveControllerSession>().Property(c => c.EndTime).HasColumnName("end_time");
//         // builder.Entity<ActiveControllerSession>().Property(c => c.LastUpdated).HasColumnName("last_updated");
//         // builder.Entity<ActiveControllerSession>().Property(c => c.Duration).HasColumnName("duration");
//         // builder.Entity<ActiveControllerSession>().Property(c => c.DatafeedFirstSeen).HasColumnName("datafeed_first");
//         // builder.Entity<ActiveControllerSession>().Property(c => c.DatafeedLastSeen).HasColumnName("datafeed_last");
//         // builder.Entity<ActiveControllerSession>().Property(c => c.IsActive).HasColumnName("is_active");
//         // builder.Entity<ActiveControllerSession>().Property(c => c.Cid).HasColumnName("cid");
//         // builder.Entity<ActiveControllerSession>().Property(c => c.PositionSimpleCallsign)
//         //     .HasColumnName("position_simple_callsign");
//         // builder.Entity<ActiveControllerSession>().Property(c => c.ConnectedCallsign)
//         //     .HasColumnName("connected_callsign");
//         // builder.Entity<ActiveControllerSession>().Property(c => c.ConnectedFrequency)
//         //     .HasColumnName("connected_frequency");
//         // builder.Entity<ActiveControllerSession>().Property(c => c.PositionSessionId)
//         //     .HasColumnName("position_session_id");
//         // builder.Entity<ActiveControllerSession>().Property(c => c.IsPositionSessionActive)
//         //     .HasColumnName("position_session_is_active");
//         //
//         // // Completed Controller Session
//         // builder.Entity<EndedControllerSession>().ToTable("completed_controller_sessions");
//         // builder.Entity<EndedControllerSession>().HasKey(c => new { c.Id, c.IsActive });
//         // builder.Entity<EndedControllerSession>().Property(c => c.Id).HasColumnName("id");
//         // builder.Entity<EndedControllerSession>().Property(c => c.StartTime).HasColumnName("start_time");
//         // builder.Entity<EndedControllerSession>().Property(c => c.EndTime).HasColumnName("end_time");
//         // builder.Entity<EndedControllerSession>().Property(c => c.LastUpdated).HasColumnName("last_updated");
//         // builder.Entity<EndedControllerSession>().Property(c => c.Duration).HasColumnName("duration");
//         // builder.Entity<EndedControllerSession>().Property(c => c.DatafeedFirstSeen).HasColumnName("datafeed_first");
//         // builder.Entity<EndedControllerSession>().Property(c => c.DatafeedLastSeen).HasColumnName("datafeed_last");
//         // builder.Entity<EndedControllerSession>().Property(c => c.IsActive).HasColumnName("is_active");
//         // builder.Entity<EndedControllerSession>().Property(c => c.Cid).HasColumnName("cid");
//         // builder.Entity<EndedControllerSession>().Property(c => c.PositionSimpleCallsign)
//         //     .HasColumnName("position_simple_callsign");
//         // builder.Entity<EndedControllerSession>().Property(c => c.ConnectedCallsign)
//         //     .HasColumnName("connected_callsign");
//         // builder.Entity<EndedControllerSession>().Property(c => c.ConnectedFrequency)
//         //     .HasColumnName("connected_frequency");
//         // builder.Entity<EndedControllerSession>().Property(c => c.PositionSessionId)
//         //     .HasColumnName("position_session_id");
//         // builder.Entity<EndedControllerSession>().Property(c => c.IsPositionSessionActive)
//         //     .HasColumnName("position_session_is_active");
//         //
//         // // Active Position Session
//         // builder.Entity<ActivePositionSession>().ToTable("active_position_sessions");
//         // builder.Entity<ActivePositionSession>().HasKey(p => new { p.Id, p.IsActive });
//         // builder.Entity<ActivePositionSession>().Property(p => p.Id).HasColumnName("id");
//         // builder.Entity<ActivePositionSession>().Property(p => p.StartTime).HasColumnName("start_time");
//         // builder.Entity<ActivePositionSession>().Property(p => p.EndTime).HasColumnName("end_time");
//         // builder.Entity<ActivePositionSession>().Property(p => p.LastUpdated).HasColumnName("last_updated");
//         // builder.Entity<ActivePositionSession>().Property(p => p.Duration).HasColumnName("duration");
//         // builder.Entity<ActivePositionSession>().Property(p => p.DatafeedFirstSeen).HasColumnName("datafeed_first");
//         // builder.Entity<ActivePositionSession>().Property(p => p.DatafeedLastSeen).HasColumnName("datafeed_last");
//         // builder.Entity<ActivePositionSession>().Property(p => p.IsActive).HasColumnName("is_active");
//         // builder.Entity<ActivePositionSession>().Property(p => p.PositionSimpleCallsign)
//         //     .HasColumnName("position_simple_callsign");
//         //
//         // // Ended Position Session
//         // builder.Entity<EndedPositionSession>().ToTable("completed_position_sessions");
//         // builder.Entity<EndedPositionSession>().HasKey(p => new { p.Id, p.IsActive });
//         // builder.Entity<EndedPositionSession>().Property(p => p.Id).HasColumnName("id");
//         // builder.Entity<EndedPositionSession>().Property(p => p.StartTime).HasColumnName("start_time");
//         // builder.Entity<EndedPositionSession>().Property(p => p.EndTime).HasColumnName("end_time");
//         // builder.Entity<EndedPositionSession>().Property(p => p.LastUpdated).HasColumnName("last_updated");
//         // builder.Entity<EndedPositionSession>().Property(p => p.Duration).HasColumnName("duration");
//         // builder.Entity<EndedPositionSession>().Property(p => p.DatafeedFirstSeen).HasColumnName("datafeed_first");
//         // builder.Entity<EndedPositionSession>().Property(p => p.DatafeedLastSeen).HasColumnName("datafeed_last");
//         // builder.Entity<EndedPositionSession>().Property(p => p.IsActive).HasColumnName("is_active");
//         // builder.Entity<EndedPositionSession>().Property(p => p.PositionSimpleCallsign)
//         //     .HasColumnName("position_simple_callsign");
//
//         builder.Entity<DatafeedRecord>(entity =>
//         {
//             entity.ToTable("datafeed_records");
//             entity.HasKey(e => e.Id);
//             entity.Property(e => e.Id)
//                 .UseIdentityAlwaysColumn()
//                 .HasColumnName("id");
//             entity.Property(e => e.NumTrackedControllerSessions).HasColumnName("num_tracked_controller_sessions");
//             entity.Property(e => e.NumTrackedPositionSessions).HasColumnName("num_tracked_position_sessions");
//             entity.Property(e => e.UpdateTimestamp).HasColumnName("update");
//         });
//     }
// }

